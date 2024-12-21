use axum::response::{IntoResponse, Redirect, Response};
use axum_login::AuthSession;
use hyper::StatusCode;
use tracing::error;

use super::AppState;

pub async fn post(mut auth_session: AuthSession<AppState>) -> Response {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/app/sign-in").into_response(),
        Err(err) => {
            error!("{:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
