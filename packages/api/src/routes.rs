use {
    crate::AppState,
    axum::{
        body::Body,
        response::IntoResponse,
        routing::{get, post},
        Router,
    },
    hyper::{header, StatusCode},
    serde::{Deserialize, Serialize},
    tower_sessions::Session,
};

mod api;
mod app_external;
mod app_internal;
mod http_to_https_redirect;
mod me;
mod sign_in;
mod sign_out;
mod sign_up;
mod sign_up_token;
mod well_known_acme_challenge;
mod well_known_openapi_json;

#[cfg(feature = "app_external")]
use app_external as app;

#[cfg(feature = "app_internal")]
use app_internal as app;

pub struct AppRouter {}

const COUNTER_KEY: &str = "counter";

#[derive(Default, Deserialize, Serialize)]
struct Counter(usize);

impl AppRouter {
    pub fn http() -> Router<AppState> {
        Router::new()
            .route(
                "/.well-known/acme-challenge",
                get(well_known_acme_challenge::get),
            )
            .fallback(http_to_https_redirect::all)
    }
    pub fn https() -> Router<AppState> {
        Router::new()
            .route(
                "/_session",
                get(|session: Session| async move {
                    let counter: Counter =
                        session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
                    session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
                    (
                        StatusCode::OK,
                        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
                        Body::new(counter.0.to_string()),
                    )
                        .into_response()
                }),
            )
            .route(
                "/.well-known/openapi.json",
                get(well_known_openapi_json::get),
            )
            .route("/api/@me", get(me::get))
            .route("/api/sign-in", post(sign_in::post))
            .route("/api/sign-out", post(sign_out::post))
            .route("/api/sign-up", post(sign_up::post))
            .route("/api/sign-up/:token", post(sign_up_token::post))
            .route("/", post(api::post))
            .route("/:key", post(api::post))
            .route("/:key", get(api::get))
            .nest("/", app::router())
    }
}
