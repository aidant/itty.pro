use {
    super::AppState,
    crate::{
        store_user::{NewUserCredentials, UserStoreExt},
        AppError,
    },
    axum::{
        extract::State,
        response::{IntoResponse, Redirect, Response},
        Form,
    },
    axum_login::AuthSession,
    hyper::StatusCode,
};

pub async fn post(
    State(state): State<AppState>,
    mut auth_session: AuthSession<AppState>,
    Form(credentials): Form<NewUserCredentials>,
) -> Result<Response, AppError> {
    let user = state.new_user(credentials).await?;

    if auth_session.login(&user).await.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    Ok(Redirect::to("/app").into_response())
}
