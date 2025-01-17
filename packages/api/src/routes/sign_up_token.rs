use {
    super::AppState,
    crate::{store_user::UserStoreExt, AppError},
    axum::{
        extract::{Path, State},
        response::{IntoResponse, Redirect, Response},
    },
    serde::Deserialize,
};

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

pub async fn post(
    Path(token): Path<String>,
    State(state): State<AppState>,
) -> Result<Response, AppError> {
    state.set_user_email_verified(&token).await?;

    Ok(Redirect::to("/app").into_response())
}
