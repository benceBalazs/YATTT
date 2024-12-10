use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthApiResponse {
    test: String,
}

pub async fn auth_token_handler() -> Json<AuthApiResponse> {
    // TODO complete path, currently test string for route
    let test = "TOKEN".to_string(); 
    Json(AuthApiResponse { test })
}

pub async fn auth_login_handler() -> Json<AuthApiResponse> {
    // TODO complete path, currently test string for route
    let test = "LOGIN".to_string();
    Json(AuthApiResponse { test })
}

pub async fn auth_register_handler() -> Json<AuthApiResponse> {
    // TODO complete path, currently test string for route
    let test = "REGISTER".to_string();
    Json(AuthApiResponse { test })
}