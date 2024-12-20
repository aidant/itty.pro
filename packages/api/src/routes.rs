use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod api;
mod app_external;
mod app_internal;
mod http_to_https_redirect;
mod well_known_acme_challenge;
mod well_known_openapi_json;

#[cfg(feature = "app_external")]
use app_external as app;

#[cfg(feature = "app_internal")]
use app_internal as app;

pub struct AppRouter {}

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
                "/.well-known/openapi.json",
                get(well_known_openapi_json::get),
            )
            .route("/", post(api::post))
            .route("/:key", post(api::post))
            .route("/:key", get(api::get))
            .nest("/", app::router())
    }
}
