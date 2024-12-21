use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_login::AuthSession;
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use password_auth::generate_hash;
use serde::Deserialize;
use tokio::task;
use uuid::Uuid;

use crate::{util::uuid_to_ms, util_auth::User, AppError};

use super::AppState;

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

pub async fn post(
    State(state): State<AppState>,
    mut auth_session: AuthSession<AppState>,
    Form(creds): Form<Credentials>,
) -> Result<Response, AppError> {
    let id = Uuid::now_v7();
    let now_ms = uuid_to_ms(&id)?;
    let password_hash = task::spawn_blocking(|| generate_hash(creds.password))
        .await
        .unwrap();

    let user = sqlx::query_as!(
        User,
        r#"
            insert into user (id, email, display_name, password, created_at, updated_at) values (?, ?, ?, ?, ?, ?)
            returning
                id as "id: Uuid",
                email,
                display_name,
                password,
                created_at as "created_at: DateTime<Utc>",
                updated_at as "updated_at: DateTime<Utc>"
        "#,
        id,
        creds.email,
        creds.display_name,
        password_hash,
        now_ms,
        now_ms,
    )
    .fetch_one(&state.conn)
    .await?;

    if auth_session.login(&user).await.is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    Ok(Redirect::to("/app").into_response())
}
