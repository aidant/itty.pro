use {
    super::AppState,
    axum::response::{IntoResponse, Redirect, Response},
    axum_login::AuthSession,
    hyper::StatusCode,
    tracing::error,
};

pub async fn post(mut auth_session: AuthSession<AppState>) -> Response {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/app").into_response(),
        Err(err) => {
            error!("{:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
