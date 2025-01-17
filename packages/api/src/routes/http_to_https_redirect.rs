use {
    anyhow::anyhow,
    axum::{
        extract::Host,
        http::{StatusCode, Uri},
        response::{IntoResponse, Redirect, Response},
    },
    tracing::error,
    url::Url,
};

fn http_to_https(host: String, uri: Uri) -> Result<String, anyhow::Error> {
    let mut parts = uri.into_parts();

    parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

    if parts.path_and_query.is_none() {
        parts.path_and_query = Some("/".parse().unwrap());
    }

    parts.authority = Some(host.parse()?);

    let mut url = Url::parse(&Uri::from_parts(parts)?.to_string())?;

    url.set_scheme("https")
        .map_err(|_| anyhow!("unable to set scheme"))?;
    url.set_port(Some(3000))
        .map_err(|_| anyhow!("unable to set port"))?;

    Ok(url.to_string())
}

pub async fn all(Host(host): Host, uri: Uri) -> Response {
    match http_to_https(host, uri) {
        Ok(url) => Redirect::permanent(&url).into_response(),
        Err(err) => {
            error!("{:?}", err);
            (StatusCode::BAD_REQUEST).into_response()
        }
    }
}
