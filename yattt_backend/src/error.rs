#[derive(Debug, serde::Serialize)]
pub struct ApiError {
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound,
    BadRequest,
    InternalServerError,
    Unauthorized
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(detail) => (
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", detail),
            ),
            AppError::NotFound => (hyper::StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::BadRequest => (hyper::StatusCode::BAD_REQUEST, "Bad request".to_string()),
            AppError::InternalServerError => (
                hyper::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            AppError::Unauthorized => (hyper::StatusCode::UNAUTHORIZED, "Not Authorized".to_string()),
        };
        let error_response = ApiError {
            message: error_message,
        };
        (status, axum::Json(error_response)).into_response()
    }
}