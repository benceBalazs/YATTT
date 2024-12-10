use axum::Json;
use chrono::Local;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthApiResponse {
    test: String,
}

pub async fn auth_handler() -> Json<AuthApiResponse> {
    let test = crate::API_VERSION.to_string(); // Define your API version here

    Json(AuthApiResponse { test })
}