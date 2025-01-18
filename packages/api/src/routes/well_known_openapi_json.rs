use {
    super::{api, me, sign_in, sign_out, sign_up, sign_up_token},
    axum::{
        response::{IntoResponse, Response},
        Json,
    },
    utoipa::OpenApi,
};

#[derive(OpenApi)]
#[openapi(paths(
    api::post,
    api::get,
    me::get,
    sign_in::post,
    sign_out::post,
    sign_up::post,
    sign_up_token::post,
))]
pub struct OpenApiSchema;

pub async fn get() -> Response {
    Json(OpenApiSchema::openapi()).into_response()
}
