use {
    super::AppState,
    crate::store_user::UserCredentials,
    axum::{
        response::{IntoResponse, Redirect, Response},
        Form,
    },
    axum_login::AuthSession,
    hyper::StatusCode,
    tracing::error,
};

pub async fn post(
    mut auth_session: AuthSession<AppState>,
    Form(credentials): Form<UserCredentials>,
) -> Response {
    let user = match auth_session.authenticate(credentials.clone()).await {
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
        error!("login failed");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/app").into_response()
}
