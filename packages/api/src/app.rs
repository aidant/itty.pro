pub struct App {}

#[cfg(not(any(feature = "app_external", feature = "app_internal")))]
pub mod app_impl {
    use super::*;
    use crate::AppState;
    use axum::Router;

    impl App {
        pub fn new() -> Router<AppState> {
            Router::new()
        }
    }
}

#[cfg(feature = "app_external")]
pub mod api_impl {
    use super::*;
    use crate::AppState;
    use axum::{
        extract::Path,
        http::{self, header},
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use hyper::StatusCode;

    async fn serve(path: &str) -> Response {
        reqwest::get(format!("http://localhost:5173/app/{}", path))
            .await
            .map(|response| Into::<http::Response<_>>::into(response).into_response())
            .unwrap_or_else(|error| {
                println!("{:?}", error);

                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
            })
    }

    impl App {
        pub fn new() -> Router<AppState> {
            Router::new()
                .route(
                    "/app",
                    get(|| async {
                        (
                            StatusCode::TEMPORARY_REDIRECT,
                            [(header::LOCATION, "/app/")],
                        )
                            .into_response()
                    }),
                )
                .route("/app/", get(|| async { serve("").await }))
                .route(
                    "/app/*path",
                    get(|Path(path): Path<String>| async move { serve(&path).await }),
                )
        }
    }
}

#[cfg(feature = "app_internal")]
pub mod app_impl {
    use super::*;
    use crate::AppState;
    use axum::{
        extract::Path,
        http::{header, StatusCode},
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use include_dir::{include_dir, Dir};

    static APP: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../app/build/");

    fn serve(path: &str) -> Response {
        APP.get_file(path).map_or_else(
            || (StatusCode::NOT_FOUND).into_response(),
            |file| {
                (
                    StatusCode::OK,
                    [(
                        header::CONTENT_TYPE,
                        mime_guess::from_path(&path)
                            .first_or_octet_stream()
                            .to_string(),
                    )],
                    file.contents(),
                )
                    .into_response()
            },
        )
    }

    impl App {
        pub fn new() -> Router<AppState> {
            Router::new()
                .route(
                    "/app",
                    get(|| async {
                        (
                            StatusCode::TEMPORARY_REDIRECT,
                            [(header::LOCATION, "/app/")],
                        )
                            .into_response()
                    }),
                )
                .route("/app/", get(|| async { serve("index.html") }))
                .route(
                    "/app/*path",
                    get(|Path(path): Path<String>| async move { serve(&path) }),
                )
        }
    }
}
