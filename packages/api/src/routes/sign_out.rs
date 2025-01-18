use {
    super::AppState,
    crate::util_app_error::{AppError, InternalServerError},
    axum::response::{IntoResponse, Response},
    axum_login::AuthSession,
    hyper::StatusCode,
};

#[utoipa::path(
    post,
    path = "/api/sign-out",
    operation_id = "sign_out",
    tag = "auth",
    responses(
        (status = 200),
        (status = 500, body = AppError)
    )
)]
pub async fn post(mut auth_session: AuthSession<AppState>) -> Result<Response, AppError> {
    auth_session
        .logout()
        .await
        .map_err(|err| InternalServerError(err.into()))?;

    Ok(StatusCode::OK.into_response())
}
