use axum::{Extension, Json};
use chrono::DateTime;
use hyper::StatusCode;
use crate::models::attendance::AttendanceResponse;

#[derive(Clone)]
pub struct CurrentUser {
    pub username: String,
    pub password_hash: String
}

// TODO documentation
pub async fn attendance_create_handler(Json(payload): Json<AttendanceResponse>) -> (StatusCode, String) {
    let mut response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO create attendance by utilizing payload

    (response, "success".to_string())
}

// TODO documentation
pub async fn attendance_retrieve_handler(Extension(user_data): Extension<CurrentUser>) -> Json<AttendanceResponse> {
    let testattendance= AttendanceResponse {
      user: user_data.username.to_string(),
      device_id: "device_id".to_string(),
      check_in_time: DateTime::from_timestamp_nanos(420420),
      check_out_time: Some(DateTime::from_timestamp_nanos(420420)), 
      duration: Some(1.0),
    };
    // TODO fill response with normal data
    
    Json(testattendance)
}