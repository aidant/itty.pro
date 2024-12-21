use axum::{
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_login::AuthSession;
use hyper::StatusCode;
use tracing::error;

use crate::util_auth::Credentials;

use super::AppState;

pub async fn post(
    mut auth_session: AuthSession<AppState>,
    Form(creds): Form<Credentials>,
) -> Response {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (StatusCode::UNAUTHORIZED).into_response();
        }
        Err(err) => {
            error!("{:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/app").into_response()
}
