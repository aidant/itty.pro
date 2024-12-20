use axum::extract::Path;

pub async fn get(Path(token): Path<String>) {}
