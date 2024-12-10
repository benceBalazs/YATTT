use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

use crate::models::user::User;

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

pub async fn auth_register_handler(Json(user): Json<User>) -> impl IntoResponse {
    let Some(created_user_id) = crate::db::surrealdb::create_user(user).await else {
        return (
            hyper::StatusCode::BAD_REQUEST,
            Json(json!({ "error": "User not created" })),
        );
    };

    // TODO return token info with id of created user instead

    (
        hyper::StatusCode::CREATED,
        Json(json!(created_user_id)),
    )
}
