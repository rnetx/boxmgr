use std::{
    fs,
    sync::{atomic::Ordering, Arc},
};

use axum::{http::StatusCode, response::IntoResponse};
use tokio::sync::mpsc;

use crate::{database, manager::Manager};

use super::generic;

// Start Service: GET ../service/start
pub(crate) async fn start_service(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match ctx.manager.get_service().start_service().await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::ErrorResponse::new(StatusCode::SERVICE_UNAVAILABLE, e.to_string())
            .into_response(),
    }
}

// Stop Service: GET ../service/stop
pub(crate) async fn stop_service(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match ctx.manager.get_service().stop_service().await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::ErrorResponse::new(StatusCode::SERVICE_UNAVAILABLE, e.to_string())
            .into_response(),
    }
}

// Restart Service: GET ../service/restart
pub(crate) async fn restart_service(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match ctx.manager.get_service().restart_service().await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::ErrorResponse::new(StatusCode::SERVICE_UNAVAILABLE, e.to_string())
            .into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct CorePathRequestBody {
    path: String,
}

// Set Core Path: PUT ../service/core_path
pub(crate) async fn set_core_path(
    ctx: generic::RequestJsonContext<(), CorePathRequestBody>,
) -> impl IntoResponse {
    let path = ctx.body.0.path;
    match database::set_core_path(&ctx.manager.get_database(), path).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Serialize)]
pub(crate) struct UploadCoreResponseBody {
    path: String,
}

// Upload Core: POST ../service/core_path
pub(crate) async fn upload_core_path(mut ctx: generic::RequestMultiPartContext) -> impl IntoResponse {
    match ctx.multipart.next_field().await {
        Ok(Some(field)) => {
            let filename = field.file_name().unwrap_or("sing-box").to_string();
            let data = match field.bytes().await {
                Ok(v) => v,
                Err(e) => {
                    return generic::GenericResponse::new(
                        StatusCode::BAD_REQUEST,
                        format!("failed to receive file: {}", e),
                    )
                    .into_response();
                }
            };
            let filename_temp = filename.clone() + ".temp";
            let filepath = ctx.manager.get_data_dir_path().join(&filename);
            let filepath_temp = ctx.manager.get_data_dir_path().join(&filename_temp);
            if let Err(e) = fs::write(&filepath_temp, data) {
                fs::remove_file(filepath_temp).ok();
                return generic::GenericResponse::new(
                    StatusCode::BAD_REQUEST,
                    format!("failed to write file: {}", e),
                )
                .into_response();
            }
            if let Err(e) = fs::rename(&filepath_temp, &filepath) {
                fs::remove_file(filepath_temp).ok();
                return generic::GenericResponse::new(
                    StatusCode::BAD_REQUEST,
                    format!("failed to rename file: {}", e),
                )
                .into_response();
            }
            generic::GenericResponse::new(
                StatusCode::OK,
                UploadCoreResponseBody {
                    path: filepath.to_string_lossy().to_string(),
                },
            )
            .into_response()
        }
        Ok(None) => {
            generic::GenericResponse::new(StatusCode::BAD_REQUEST, "missing file in multipart form")
                .into_response()
        }
        Err(e) => generic::GenericResponse::new(
            StatusCode::BAD_REQUEST,
            format!("failed to receive file: {}", e),
        )
        .into_response(),
    }
}

// Get Config: GET ../service/config
pub(crate) async fn get_config(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    let config = ctx.manager.get_service().get_config().await;
    generic::GenericResponse::new(StatusCode::OK, config).into_response()
}

#[derive(serde::Serialize)]
pub(crate) struct CorePathResponseBody {
    path: Option<String>,
}

// Get Core Path: GET ../service/core_path
pub(crate) async fn get_core_path(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match database::get_core_path(&ctx.manager.get_database()).await {
        Ok(path) => generic::GenericResponse::new(StatusCode::OK, CorePathResponseBody { path })
            .into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

// Get Auto Start: GET ../service/auto_start
pub(crate) async fn get_auto_start(ctx: generic::RequestRawBodyContext) -> impl IntoResponse {
    match database::get_auto_start(&ctx.manager.get_database()).await {
        Ok(b) => generic::GenericResponse::new(StatusCode::OK, b).into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct AutoStartRequestBody {
    status: bool,
}

// Set Auto Start: PUT ../service/auto_start
pub(crate) async fn set_auto_start(
    ctx: generic::RequestJsonContext<(), AutoStartRequestBody>,
) -> impl IntoResponse {
    match database::set_auto_start(&ctx.manager.get_database(), ctx.body.0.status).await {
        Ok(_) => generic::GenericResponse::new(StatusCode::OK, "success").into_response(),
        Err(e) => generic::db_error_to_http_response(e).into_response(),
    }
}

#[derive(serde::Serialize)]
pub(crate) struct StatusResponse {
    is_running: bool,
    running_config: String,
    core_version: String,
    memory_usage: u64,
    connection_count: usize,
    upload_traffic: u64,
    download_traffic: u64,
    upload_speed: u64,
    download_speed: u64,
}

// Set Status: (Websocket) ../service/status
pub(crate) async fn get_status(
    ws: axum::extract::ws::WebSocketUpgrade,
    state: axum::extract::State<Arc<Manager>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| {
        let service = state.get_service();
        async move {
            let (notify, status) = service.get_status();
            loop {
                let response = StatusResponse {
                    is_running: status.is_running.load(Ordering::Relaxed),
                    running_config: status.running_config.read().unwrap().clone(),
                    core_version: status.core_version.read().unwrap().clone(),
                    memory_usage: status.memory_usage.load(Ordering::Relaxed),
                    connection_count: status.connection_count.load(Ordering::Relaxed),
                    upload_traffic: status.upload_traffic.load(Ordering::Relaxed),
                    download_traffic: status.download_traffic.load(Ordering::Relaxed),
                    upload_speed: status.upload_speed.load(Ordering::Relaxed),
                    download_speed: status.download_speed.load(Ordering::Relaxed),
                };
                let s = serde_json::json!(response).to_string();
                if let Err(_) = socket.send(axum::extract::ws::Message::Text(s)).await {
                    let _ = socket.send(axum::extract::ws::Message::Close(None)).await;
                    return;
                }
                notify.notified().await;
            }
        }
    })
}

// Log: (Websocket) ../service/log
pub(crate) async fn get_log(
    ws: axum::extract::ws::WebSocketUpgrade,
    state: axum::extract::State<Arc<Manager>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| {
        let service = state.get_service();
        async move {
            let log_queue_listener = service.log_queue_listener();
            let (sender, mut receiver) = mpsc::channel(1);
            tokio::spawn(async move {
                log_queue_listener.listen(sender).await;
            });
            while let Some(s) = receiver.recv().await {
                if let Err(_) = socket.send(axum::extract::ws::Message::Text(s)).await {
                    break;
                }
            }
            let _ = socket.send(axum::extract::ws::Message::Close(None)).await;
        }
    })
}
