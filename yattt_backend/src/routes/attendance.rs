use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct AttendanceApiResponse {
    test: String,
}

pub async fn attendance_create_handler() -> Json<AttendanceApiResponse> {
    // TODO complete path, currently test string for route
    let test = "CREATE".to_string();
    Json(AttendanceApiResponse { test })
}

pub async fn attendance_retrieve_handler() -> Json<AttendanceApiResponse> {
    // TODO complete path, currently test string for route
    let test = "RETRIEVE".to_string(); 
    Json(AttendanceApiResponse { test })
}