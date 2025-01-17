use {
    axum::{
        extract::Host,
        http::{
            uri::{InvalidUri, InvalidUriParts},
            StatusCode, Uri,
        },
        response::{IntoResponse, Redirect, Response},
    },
    thiserror::Error,
    tracing::error,
    url::Url,
};

#[derive(Error, Debug)]
pub enum HttpToHttpsError {
    #[error(transparent)]
    InvalidUri(#[from] InvalidUri),
    #[error(transparent)]
    InvalidUriParts(#[from] InvalidUriParts),
    #[error(transparent)]
    ParseError(#[from] url::ParseError),
    #[error("unable to set scheme")]
    Scheme,
    #[error("unable to set port")]
    Port,
}

fn http_to_https(host: String, uri: Uri) -> Result<String, HttpToHttpsError> {
    let mut parts = uri.into_parts();

    parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

    if parts.path_and_query.is_none() {
        parts.path_and_query = Some("/".parse().unwrap());
    }

    parts.authority = Some(host.parse()?);

    let mut url = Url::parse(&Uri::from_parts(parts)?.to_string())?;

    url.set_scheme("https")
        .map_err(|_| HttpToHttpsError::Scheme)?;
    url.set_port(Some(3000))
        .map_err(|_| HttpToHttpsError::Port)?;

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
