use axum::{http::StatusCode, response::IntoResponse};

use super::generic;

use crate::database;

// Get Config: GET ../config/:id
pub(crate) async fn get_config(ctx: generic::RequestRawBodyContext<String>) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    match database::get_config(&ctx.manager.get_database(), id).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct ModifyConfigRequestBody {
    tag: Option<String>,
    config: Option<serde_json::Value>,
}

// Modify Config: PATCH ../config/:id
pub(crate) async fn modify_config(
    ctx: generic::RequestJsonContext<String, ModifyConfigRequestBody>,
) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    match (&ctx.body.0.tag, &ctx.body.0.config) {
        (None, None) => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing tag and config")
                .into_response();
        }
        _ => {}
    }
    let mut config = database::ActiveConfig::default();
    if let Some(v) = ctx.body.0.tag {
        config.tag = sea_orm::ActiveValue::Set(v);
    }
    if let Some(v) = ctx.body.0.config {
        config.config = sea_orm::ActiveValue::Set(v);
    }
    match database::modify_config(&ctx.manager.get_database(), id, config).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AddConfigRequestBody {
    tag: String,
    config: serde_json::Value,
}

// Add Config: POST ../config
pub(crate) async fn add_config(
    ctx: generic::RequestJsonContext<(), AddConfigRequestBody>,
) -> impl IntoResponse {
    let config = database::Config {
        id: String::new(),
        tag: ctx.body.0.tag,
        config: ctx.body.0.config,
        actived: false,
    };
    match database::add_config(&ctx.manager.get_database(), config).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// Delete Config: DELETE ../config/:id
pub(crate) async fn delete_config(
    ctx: generic::RequestRawBodyContext<String>,
) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    match database::delete_config(&ctx.manager.get_database(), id).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct BulkDeleteRequestBody {
    ids: Vec<String>,
}

// Bulk Delete Config: POST ../bluk_config_delete
pub(crate) async fn bulk_delete_config(
    ctx: generic::RequestJsonContext<(), BulkDeleteRequestBody>,
) -> impl IntoResponse {
    match database::bulk_delete_config(&ctx.manager.get_database(), ctx.body.0.ids).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// List Config: GET ../config (params: ?simple=<bool>)
pub(crate) async fn list_config(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match database::list_config(&ctx.manager.get_database()).await {
        Ok(mut v) => {
            if let Some(pq) = ctx.req.uri().path_and_query() {
                if let Some(q) = pq.query() {
                    if q.contains("simple=true") {
                        v = v
                            .into_iter()
                            .map(|mut c| {
                                c.config = serde_json::Value::Null;
                                c
                            })
                            .collect();
                    }
                }
            }
            generic::GenericResponse::new(StatusCode::OK, v).into_response()
        }
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// Set Active Config: PUT ../active_config/:id
pub(crate) async fn set_active_config(
    ctx: generic::RequestRawBodyContext<String>,
) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    match database::set_active_config(&ctx.manager.get_database(), id).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// Get Active Config: GET ../active_config
pub(crate) async fn get_active_config(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match database::get_active_config(&ctx.manager.get_database()).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}
