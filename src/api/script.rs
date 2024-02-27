use axum::{http::StatusCode, response::IntoResponse};

use super::generic;

use crate::database;

// Get Script: GET ../script/:id
pub(crate) async fn get_script(ctx: generic::RequestRawBodyContext<String>) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    match database::get_script(&ctx.manager.get_database(), id).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct ModifyScriptRequestBody {
    tag: Option<String>,
    content: Option<String>,
}

// Modify Script: PATCH ../script/:id
pub(crate) async fn modify_script(
    ctx: generic::RequestJsonContext<String, ModifyScriptRequestBody>,
) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    let mut script = database::ActiveScript::default();
    if let Some(v) = ctx.body.0.tag {
        script.tag = sea_orm::ActiveValue::Set(v);
    }
    if let Some(v) = ctx.body.0.content {
        script.content = sea_orm::ActiveValue::Set(v);
    }
    match database::modify_script(&ctx.manager.get_database(), id, script).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AddScriptRequestBody {
    tag: String,
    content: String,
}

// Add Script: POST ../script
pub(crate) async fn add_script(
    ctx: generic::RequestJsonContext<(), AddScriptRequestBody>,
) -> impl IntoResponse {
    let script = database::Script {
        id: String::new(),
        tag: ctx.body.0.tag,
        content: ctx.body.0.content,
        run_type: 0,
    };
    match database::add_script(&ctx.manager.get_database(), script).await {
        Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// Delete Script: DELETE ../script/:id
pub(crate) async fn delete_script(
    ctx: generic::RequestRawBodyContext<String>,
) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    match database::delete_script(&ctx.manager.get_database(), id).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct BulkDeleteRequestBody {
    ids: Vec<String>,
}

// Bulk Delete Script: POST ../bluk_script_delete
pub(crate) async fn bulk_delete_script(
    ctx: generic::RequestJsonContext<(), BulkDeleteRequestBody>,
) -> impl IntoResponse {
    match database::bulk_delete_script(&ctx.manager.get_database(), ctx.body.0.ids).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// List Script: GET ../script (params: ?simple=<bool>)
pub(crate) async fn list_script(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match database::list_script(&ctx.manager.get_database()).await {
        Ok(mut v) => {
            if let Some(pq) = ctx.req.uri().path_and_query() {
                if let Some(q) = pq.query() {
                    if q.contains("simple=true") {
                        v = v
                            .into_iter()
                            .map(|mut s| {
                                s.content = String::new();
                                s
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

// Clean Script Run Type: DELETE ../script_run_type/:id
pub(crate) async fn clean_script_run_type(
    ctx: generic::RequestRawBodyContext<String>,
) -> impl IntoResponse {
    let id = match ctx.path_params {
        Some(p) => p.0,
        None => {
            return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                .into_response();
        }
    };
    match database::clean_script_type(&ctx.manager.get_database(), id).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[macro_export]
macro_rules! api_set_run_type_script_macro {
    ($name:ident, $db_name:expr) => {
        // Set $label Script: PUT ../$label/:id
        pub(crate) async fn $name(
            ctx: generic::RequestRawBodyContext<String>,
        ) -> impl IntoResponse {
            let id = match ctx.path_params {
                Some(p) => p.0,
                None => {
                    return generic::ErrorResponse::new(StatusCode::BAD_REQUEST, "missing id")
                        .into_response();
                }
            };
            match $db_name(&ctx.manager.get_database(), id).await {
                Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
                Err(e) => generic::db_error_to_http_response(e).into_response(),
            }
        }
    };
}

#[macro_export]
macro_rules! api_get_run_type_script_macro {
    ($name:ident, $db_name:expr) => {
        // Get $label Script: GET ../$label
        pub(crate) async fn $name(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
            match $db_name(&ctx.manager.get_database()).await {
                Ok(v) => generic::GenericResponse::new(StatusCode::OK, v).into_response(),
                Err(e) => generic::db_error_to_http_response(e).into_response(),
            }
        }
    };
}

api_set_run_type_script_macro!(set_before_start_script, database::set_before_start_script);
api_get_run_type_script_macro!(get_before_start_script, database::get_before_start_script);
api_set_run_type_script_macro!(set_after_start_script, database::set_after_start_script);
api_get_run_type_script_macro!(get_after_start_script, database::get_after_start_script);
api_set_run_type_script_macro!(set_before_close_script, database::set_before_close_script);
api_get_run_type_script_macro!(get_before_close_script, database::get_before_close_script);
api_set_run_type_script_macro!(set_after_close_script, database::set_after_close_script);
api_get_run_type_script_macro!(get_after_close_script, database::get_after_close_script);
