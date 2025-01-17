use {
    super::AppState,
    crate::{store_user::UserStoreExt, AppError},
    axum::{
        extract::{Path, State},
        response::{IntoResponse, Redirect, Response},
    },
};

pub async fn post(
    Path(token): Path<String>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    state.set_user_email_verified(&token).await?;

    Ok(Redirect::to("/app").into_response())
}
