use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct CardApiResponse {
    test: String,
}

pub async fn card_create_handler() -> Json<CardApiResponse> {
    // TODO complete path, currently test string for route
    let test = "CREATE".to_string();
    Json(CardApiResponse { test })
}

pub async fn card_retrieve_handler() -> Json<CardApiResponse> {
    // TODO complete path, currently test string for route
    let test = "RETRIEVE".to_string();
    Json(CardApiResponse { test })
}

pub async fn card_modify_handler() -> Json<CardApiResponse> {
    // TODO complete path, currently test string for route
    let test = "MODIFY".to_string();
    Json(CardApiResponse { test })
}

pub async fn card_delete_handler() -> Json<CardApiResponse> {
    // TODO complete path, currently test string for route
    let test = "DELETE".to_string();
    Json(CardApiResponse { test })
}