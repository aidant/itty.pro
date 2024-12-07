#[cfg(not(feature = "include_app"))]
pub mod app_impl {
    use axum::Router;

    use crate::AppState;

    pub fn app() -> Router<AppState> {
        Router::new()
    }
}

#[cfg(feature = "include_app")]
pub mod app_impl {
    use axum::{
        extract::Path,
        http::{header, StatusCode},
        response::{IntoResponse, Response},
        routing::get,
        Router,
    };
    use include_dir::{include_dir, Dir};

    use crate::AppState;

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

    pub fn app() -> Router<AppState> {
        Router::new()
            .route("/", get(|| async { serve("index.html") }))
            .route(
                "/*path",
                get(|Path(path): Path<String>| async move { serve(&path) }),
            )
    }
}
