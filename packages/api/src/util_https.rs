use axum::{extract::connect_info::IntoMakeServiceWithConnectInfo, Router};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
    service::TowerToHyperService,
};
use std::{io, net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tokio_rustls::{
    rustls::{
        pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
        server::{Acceptor, ClientHello},
        ServerConfig,
    },
    LazyConfigAcceptor,
};
use tower_service::Service;
use tracing::{error, trace};

// https://github.com/rustls/tokio-rustls/blob/cd399aba544e01f08047b40a6988365c195d6076/src/lib.rs#L225-L250
// https://github.com/tokio-rs/axum/blob/9983bc1da460becd3a0f08c513d610411e84dd43/axum/src/serve.rs#L225-L269

pub async fn serve_https(
    tcp_listener: TcpListener,
    mut make_service: IntoMakeServiceWithConnectInfo<Router, SocketAddr>,
) -> Result<(), anyhow::Error> {
    loop {
        let (tcp_stream, remote_addr) = match tcp_accept(&tcp_listener).await {
            Some(conn) => conn,
            None => continue,
        };

        let tower_service = make_service
            .call(remote_addr)
            .await
            .unwrap_or_else(|err| match err {});

        let hyper_service = TowerToHyperService::new(tower_service);
        let acceptor = LazyConfigAcceptor::new(Acceptor::default(), tcp_stream);

        tokio::spawn(async move {
            tokio::pin!(acceptor);

            let result: Result<(), anyhow::Error> = async {
                let start = acceptor.as_mut().await?;

                let client_hello = start.client_hello();
                let config = get_server_config(client_hello).await;
                let tcp_stream = start.into_stream(config).await?;

                trace!("connection {remote_addr} accepted");

                let tcp_stream = TokioIo::new(tcp_stream);

                Builder::new(TokioExecutor::new())
                    .serve_connection_with_upgrades(tcp_stream, hyper_service)
                    .await
                    .or_else(|_err| Ok(()))
            }
            .await;

            if let Err(err) = result {
                error!("unable to process request: {err}");

                if let Some(mut stream) = acceptor.take_io() {
                    stream
                        .write_all("HTTP/1.1 400 Bad Request\r\n\r\n\r\n".as_bytes())
                        .await
                        .unwrap_or_else(|err| {
                            error!("unable to send error response: {err}");
                        });
                }
            }
        });
    }
}

fn is_connection_error(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::ConnectionRefused
            | io::ErrorKind::ConnectionAborted
            | io::ErrorKind::ConnectionReset
    )
}

async fn tcp_accept(listener: &TcpListener) -> Option<(TcpStream, SocketAddr)> {
    match listener.accept().await {
        Ok(conn) => Some(conn),
        Err(e) => {
            if is_connection_error(&e) {
                return None;
            }

            error!("accept error: {e}");
            tokio::time::sleep(Duration::from_secs(1)).await;
            None
        }
    }
}

pub async fn get_server_config(client_hello: ClientHello<'_>) -> Arc<ServerConfig> {
    let sni = client_hello.server_name();

    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            CertificateDer::pem_file_iter(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("localhost-cert.pem"),
            )
            .unwrap()
            .map(|cert| cert.unwrap())
            .collect(),
            PrivateKeyDer::from_pem_file(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("localhost-key.pem"),
            )
            .unwrap(),
        )
        .unwrap();

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    Arc::new(config)
}
