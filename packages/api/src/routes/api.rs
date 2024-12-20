use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Host, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::{extract::OptionalPath, headers::UserAgent, TypedHeader};
use nanoid::nanoid;
use serde_json::json;
use url::Url;
use uuid::Uuid;

use crate::{util::uuid_to_ms, AppError};

use super::AppState;

#[utoipa::path(
  post,
  path = "/",
  request_body = String,
  responses(
      (status = 201, body = String),
      (status = 422, body = String),
      (status = 500, body = String)
  )
)]
pub async fn post(
    State(state): State<AppState>,
    Host(host): Host,
    OptionalPath(path): OptionalPath<String>,
    payload: String,
) -> Result<Response, AppError> {
    let id = Uuid::now_v7();
    let key = path.unwrap_or_else(|| nanoid!(8));
    let now_ms = uuid_to_ms(&id)?;

    let url_string = match Url::parse(&payload) {
        Ok(url) => url.to_string(),
        Err(error) => {
            return Ok((
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": error.to_string() })),
            )
                .into_response());
        }
    };

    sqlx::query!(
        r#"
        insert into url (id, key, url, created_at, updated_at) values (?, ?, ?, ?, ?)
    "#,
        id,
        key,
        url_string,
        now_ms,
        now_ms,
    )
    .execute(&state.conn)
    .await?;

    Ok((
        StatusCode::CREATED,
        Url::parse(&format!("https://{host}:3000/{key}"))?.to_string(),
    )
        .into_response())
}

#[utoipa::path(
    get,
    path = "/{key}",
    responses(
        (status = 307),
        (status = 404),
        (status = 500, body = String),
    )
)]
pub async fn get(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Path(key): Path<String>,
) -> Result<Response, AppError> {
    let row = sqlx::query!(
        r#"
            select id as "id: Uuid", url from url where key = ?
        "#,
        key
    )
    .fetch_optional(&state.conn)
    .await?;

    if let Some(row) = row {
        let id = Uuid::now_v7();
        let now_ms = uuid_to_ms(&id)?;

        let req_client_ip = addr.to_string();
        let req_user_agent = user_agent.to_string();

        sqlx::query!(
            r#"
                insert into url_analytics (id, url_id, req_client_ip, req_user_agent, created_at, updated_at) values (?, ?, ?, ?, ?, ?)
            "#,
            id,
            row.id,
            req_client_ip,
            req_user_agent,
            now_ms,
            now_ms,
        )
        .execute(&state.conn)
        .await?;

        Ok((StatusCode::TEMPORARY_REDIRECT, [("Location", row.url)]).into_response())
    } else {
        Ok((StatusCode::NOT_FOUND).into_response())
    }
}
