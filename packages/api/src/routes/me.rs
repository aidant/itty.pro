use {
    super::AppState,
    axum::{
        response::{IntoResponse, Response},
        Json,
    },
    axum_login::AuthSession,
    serde_json::json,
};

pub async fn get(mut auth_session: AuthSession<AppState>) -> Response {
    match auth_session.user {
        None => Json(json!({ "data": null })).into_response(),
        Some(user) => {
            Json(json!({ "data": { "display_name": user.display_name, "email": user.email, "email_verified": user.email_verified } }))
                .into_response()
        }
    }
}
