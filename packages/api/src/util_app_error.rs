use {
    axum::{response::IntoResponse, Json},
    hyper::StatusCode,
    serde::Serialize,
    thiserror::Error,
    tracing::debug,
    utoipa::ToSchema,
};

#[derive(Error, Debug)]
#[error(transparent)]
pub(crate) struct InternalServerError(#[from] pub anyhow::Error);

#[derive(Debug, Serialize, ToSchema)]
#[serde(tag = "error", rename_all = "snake_case")]
pub(crate) enum AppError {
    InternalServerError,
}

impl From<InternalServerError> for AppError {
    fn from(value: InternalServerError) -> Self {
        debug!("{:?}", value);
        Self::InternalServerError
    }
}

impl Into<StatusCode> for &AppError {
    fn into(self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (Into::<StatusCode>::into(&self), Json(self)).into_response()
    }
}
