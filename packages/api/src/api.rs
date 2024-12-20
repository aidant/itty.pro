use anyhow::anyhow;
use axum::{
    extract::{ConnectInfo, Host, Path, State},
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::{extract::OptionalPath, headers::UserAgent, TypedHeader};
use nanoid::nanoid;
use serde::Serialize;
use serde_json::json;
use std::net::SocketAddr;
use tracing::error;
use url::Url;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{util::uuid_to_ms, AppError, AppState};

#[derive(OpenApi)]
#[openapi(paths(api_create_url, api_visit_url))]
pub struct OpenApiSchema;

#[derive(Serialize, ToSchema)]
pub struct CreateUrlResponse {
    key: String,
}

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
pub async fn api_create_url(
    State(state): State<AppState>,
    Host(host): Host,
    OptionalPath(path): OptionalPath<String>,
    payload: String,
) -> Result<Response, AppError> {
    let id = Uuid::now_v7();
    let key = if let Some(path) = path {
        path
    } else {
        nanoid!(8)
    };
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
pub async fn api_visit_url(
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

pub struct Api {}

impl Api {
    pub fn new() -> Router<AppState> {
        Router::new()
            .route(
                "/.well-known/openapi.json",
                get(|| async { Json(OpenApiSchema::openapi()) }),
            )
            .route("/", post(api_create_url))
            .route("/:key", post(api_create_url))
            .route("/:key", get(api_visit_url))
    }
}

pub struct Insecure {}

fn http_to_https(host: String, uri: Uri) -> Result<String, anyhow::Error> {
    let mut parts = uri.into_parts();

    parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

    if parts.path_and_query.is_none() {
        parts.path_and_query = Some("/".parse().unwrap());
    }

    parts.authority = Some(host.parse()?);

    let mut url = Url::parse(&Uri::from_parts(parts)?.to_string())?;

    url.set_scheme("https")
        .map_err(|_| anyhow!("unable to set scheme"))?;
    url.set_port(Some(3000))
        .map_err(|_| anyhow!("unable to set port"))?;

    Ok(url.to_string())
}

impl Insecure {
    pub fn new() -> Router<AppState> {
        Router::new()
            .route(
                "/.well-known/acme-challenge/:token",
                get(|Path(token): Path<String>| async move {
                    let token = token.replacen("/.well-known/acme-challenge/", "", 1);
                }),
            )
            .fallback(|Host(host): Host, uri: Uri| async move {
                match http_to_https(host, uri) {
                    Ok(url) => Redirect::permanent(&url).into_response(),
                    Err(err) => {
                        error!("{:?}", err);
                        (StatusCode::BAD_REQUEST).into_response()
                    }
                }
            })
    }
}
