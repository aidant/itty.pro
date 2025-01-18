use {
    super::AppState,
    crate::{
        store_user::{NewUserCredentials, NewUserError, User, UserStoreExt},
        util_app_error::AppError,
    },
    axum::{
        extract::State,
        response::{IntoResponse, Response},
        Form, Json,
    },
    axum_login::AuthSession,
    hyper::StatusCode,
};

#[utoipa::path(
    post,
    path = "/api/sign-up",
    operation_id = "sign_up",
    tag = "auth",
    responses(
        (status = 201, body = User),
        (status = 422, body = NewUserError),
        (status = 500, body = AppError)
    )
)]
pub async fn post(
    State(state): State<AppState>,
    mut auth_session: AuthSession<AppState>,
    Form(credentials): Form<NewUserCredentials>,
) -> Result<Response, NewUserError> {
    let user = state.new_user(credentials).await?;

    auth_session
        .login(&user)
        .await
        .map_err(anyhow::Error::new)?;

    Ok((StatusCode::CREATED, Json(user)).into_response())
}
