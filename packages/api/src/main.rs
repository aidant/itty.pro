use axum::{
    extract::MatchedPath,
    http::{Request, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use serde_json::json;
use sqlx::SqlitePool;
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{
        pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
        server::{Acceptor, ClientHello},
        ServerConfig,
    },
    LazyConfigAcceptor,
};
use tower_http::trace::TraceLayer;
use tower_service::Service;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod app;
mod util;

#[derive(Clone)]
pub(crate) struct AppState {
    pub conn: SqlitePool,
}

pub(crate) struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::debug!("{}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conn = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./src/").run(&conn).await.unwrap();

    let app = Router::new()
        .nest("/app", app::App::new())
        .nest("/", api::Api::new())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!("http_request", method = ?request.method(), matched_path)
            }),
        )
        .with_state(AppState { conn });

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let mut make_service = app.into_make_service_with_connect_info::<SocketAddr>();

    loop {
        let (stream, socket) = listener.accept().await.unwrap();
        println!("socket: {}", socket);
        let tower_service = make_service.call(socket).await.unwrap();

        tokio::spawn(async move {
            let acceptor = LazyConfigAcceptor::new(Acceptor::default(), stream);
            tokio::pin!(acceptor);

            match acceptor.as_mut().await {
                Ok(start) => {
                    let client_hello = start.client_hello();
                    let config = choose_server_config(client_hello).await;
                    let stream = start.into_stream(config).await.unwrap();

                    let stream = TokioIo::new(stream);

                    let hyper_service =
                        hyper::service::service_fn(move |request: Request<Incoming>| {
                            tower_service.clone().call(request)
                        });

                    hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                        .serve_connection_with_upgrades(stream, hyper_service)
                        .await
                        .unwrap();
                }
                Err(error) => {
                    println!("error: {}", error);

                    if let Some(mut stream) = acceptor.take_io() {
                        stream
                            .write_all(
                                format!("HTTP/1.1 400 Invalid Input\r\n\r\n\r\n{:?}\r\n", error)
                                    .as_bytes(),
                            )
                            .await
                            .unwrap();
                    }
                }
            }
        });
    }
}

pub async fn choose_server_config(client_hello: ClientHello<'_>) -> Arc<ServerConfig> {
    let sni = client_hello.server_name();

    println!("sni: {:?}", sni);

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
