use {
    super::AppState,
    crate::{store_user::UserStoreExt, util_app_error::AppError},
    axum::{
        extract::{Path, State},
        response::{IntoResponse, Response},
        Json,
    },
    hyper::StatusCode,
    serde::Serialize,
    utoipa::ToSchema,
};

#[derive(Debug, Serialize, ToSchema)]
struct EmailVerificationStatus {
    email_verified: bool,
}

#[utoipa::path(
    post,
    path = "/api/sign-up/{email_verification_token}",
    operation_id = "sign_up_verify_email",
    tag = "auth",
    responses(
        (status = 200, body = EmailVerificationStatus),
        (status = 500, body = AppError)
    )
)]
pub async fn post(
    Path(token): Path<String>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    let user = state.set_user_email_verified(&token).await?;

    Ok((
        StatusCode::OK,
        Json(EmailVerificationStatus {
            email_verified: user.map_or(false, |user| user.email_verified),
        }),
    )
        .into_response())
}
