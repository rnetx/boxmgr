use axum::{http::StatusCode, response::IntoResponse};

use crate::database;

use super::generic;

#[derive(serde::Deserialize)]
pub(crate) struct KvRequestBody {
    pub(crate) value: serde_json::Value,
}

#[derive(serde::Serialize)]
pub(crate) struct KvResponseBody {
    pub(crate) key: String,
    pub(crate) value: serde_json::Value,
}

// Set Kv: POST ../kv/:key
pub(crate) async fn set_kv(
    ctx: generic::RequestJsonContext<String, KvRequestBody>,
) -> impl IntoResponse {
    let key = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing key")
                .into_response();
        }
    };
    let kv = database::Kv {
        key,
        value: ctx.body.0.value,
    };
    match database::set_kv(&ctx.manager.get_database(), kv).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// Get Kv: GET ../kv/:key
pub(crate) async fn get_kv(ctx: generic::RequestRawBodyContext<String>) -> impl IntoResponse {
    let key = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing key")
                .into_response();
        }
    };
    match database::get_kv(&ctx.manager.get_database(), key).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// Delete Kv: DELETE ../kv/:key
pub(crate) async fn delete_kv(ctx: generic::RequestRawBodyContext<String>) -> impl IntoResponse {
    let key = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing key")
                .into_response();
        }
    };
    match database::delete_kv(&ctx.manager.get_database(), key).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// List Kv: GET ../kv
pub(crate) async fn list_kv(ctx: generic::RequestRawBodyContext<()>) -> impl IntoResponse {
    match database::list_kv(&ctx.manager.get_database()).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}
