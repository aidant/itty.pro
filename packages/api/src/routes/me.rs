use {
    super::AppState,
    crate::{store_user::User, util_app_error::AppError},
    axum::{
        response::{IntoResponse, Response},
        Json,
    },
    axum_login::AuthSession,
};

#[utoipa::path(
    get,
    path = "/api/@me",
    responses(
        (status = 200, body = Option<User>),
        (status = 500, body = AppError)
    )
)]
pub async fn get(auth_session: AuthSession<AppState>) -> Response {
    Json(auth_session.user).into_response()
}
