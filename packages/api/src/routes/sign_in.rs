use {
    super::AppState,
    crate::{
        store_user::{User, UserCredentials},
        util_app_error::{AppError, InternalServerError},
    },
    axum::{
        response::{IntoResponse, Response},
        Form, Json,
    },
    axum_login::AuthSession,
    hyper::StatusCode,
};

#[utoipa::path(
    post,
    path = "/api/sign-in",
    operation_id = "sign_in",
    tag = "auth",
    responses(
        (status = 200, body = User),
        (status = 401),
        (status = 500, body = AppError)
    )
)]
pub async fn post(
    mut auth_session: AuthSession<AppState>,
    Form(credentials): Form<UserCredentials>,
) -> Result<Response, AppError> {
    match auth_session.authenticate(credentials).await {
        Ok(Some(user)) => {
            auth_session
                .login(&user)
                .await
                .map_err(|err| InternalServerError(err.into()))?;

            Ok(Json(user).into_response())
        }
        Ok(None) => Ok(StatusCode::UNAUTHORIZED.into_response()),
        Err(err) => Err(InternalServerError(err.into()).into()),
    }
}
