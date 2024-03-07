use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use http::StatusCode;

use crate::manager;

// Request to exit: GET /manager/request_to_exit
pub(crate) async fn request_to_exit(manager: State<Arc<manager::Manager>>) -> impl IntoResponse {
    manager.request_exit();
    StatusCode::NO_CONTENT.into_response()
}
