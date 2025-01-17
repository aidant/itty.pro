use {
    crate::AppState,
    axum::{
        extract::Path,
        http::{self, header},
        response::{IntoResponse, Response},
        routing::get,
        Router,
    },
    hyper::StatusCode,
};

async fn serve(path: &str) -> Response {
    reqwest::get(format!("http://localhost:5173/app/{}", path))
        .await
        .map(|response| Into::<http::Response<_>>::into(response).into_response())
        .unwrap_or_else(|error| {
            println!("{:?}", error);

            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        })
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
        .route("/app/", get(|| async { serve("").await }))
        .route(
            "/app/*path",
            get(|Path(path): Path<String>| async move { serve(&path).await }),
        )
}
