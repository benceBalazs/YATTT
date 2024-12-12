use axum::Json;
use chrono::Local;
use serde::Serialize;

#[derive(Serialize)]
pub struct RootApiResponse {
    version: String,
    local_time: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Successful re-authentication by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side - try later again")
    ),
)]
pub async fn root_handler() -> Json<RootApiResponse> {
    // Define your API version here
    let version = crate::API_VERSION.to_string();
    // Get the current local time in ISO 8601 format
    let local_time = Local::now().to_rfc3339();

    Json(RootApiResponse { version, local_time })
}