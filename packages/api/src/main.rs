use std::{env, net::SocketAddr};

use api::{api_create_url, api_visit_url, OpenApiSchema};
use axum::{
    extract::MatchedPath,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;

mod api;
mod app;
mod util;

#[derive(Clone)]
pub(crate) struct AppState {
    pub conn: SqlitePool,
}

pub(crate) struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::debug!("{}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conn = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./src/").run(&conn).await.unwrap();

    let app = Router::new()
        .route(
            "/.well-known/openapi.json",
            get(|| async { Json(OpenApiSchema::openapi()) }),
        )
        .nest("/app", app::app_impl::app())
        .route("/", post(api_create_url))
        .route("/:key", get(api_visit_url))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!("http_request", method = ?request.method(), matched_path)
            }),
        )
        .with_state(AppState { conn });

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}