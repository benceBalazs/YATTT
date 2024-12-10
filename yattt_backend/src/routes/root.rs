use axum::{routing::get, Json, Router};
use chrono::Local;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    version: String,
    local_time: String,
}

pub async fn root_handler() -> Json<ApiResponse> {
    let version = crate::API_VERSION.to_string(); // Define your API version here
    let local_time = Local::now().to_rfc3339(); // Get the current local time in ISO 8601 format

    Json(ApiResponse { version, local_time })
}