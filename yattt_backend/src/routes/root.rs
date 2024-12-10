use axum::Json;
use chrono::Local;
use serde::Serialize;

#[derive(Serialize)]
pub struct RootApiResponse {
    version: String,
    local_time: String,
}

pub async fn root_handler() -> Json<RootApiResponse> {
    // Define your API version here
    let version = crate::API_VERSION.to_string();
    // Get the current local time in ISO 8601 format
    let local_time = Local::now().to_rfc3339();

    Json(RootApiResponse { version, local_time })
}