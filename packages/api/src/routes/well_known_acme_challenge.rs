use axum::extract::Path;

pub async fn get(Path(_token): Path<String>) {}
