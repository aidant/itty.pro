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

fn serve(mut path: &str) -> Response {
    while path.ends_with("/") {
        path = &path[0..path.len() - 1];
    }

    if path == "" {
        path = &"index.html"
    }

    APP.get_file(path)
        .or_else(|| APP.get_file(format!("{path}.html")))
        .or_else(|| APP.get_file(format!("{path}/index.html")))
        .or_else(|| APP.get_file("404.html"))
        .map_or_else(
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

pub fn router() -> Router<AppState> {
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
        .route("/app/", get(|| async { serve("") }))
        .route(
            "/app/*path",
            get(|Path(path): Path<String>| async move { serve(&path) }),
        )
}
