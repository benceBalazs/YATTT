use axum::Json;
use chrono::DateTime;
use hyper::StatusCode;
use crate::models::attendance::AttendanceResponse;

// TODO documentation
pub async fn attendance_create_handler(Json(payload): Json<AttendanceResponse>) -> Json<StatusCode> {
    let mut response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO create attendance by utilizing payload

    Json(response)
}

// TODO documentation
pub async fn attendance_retrieve_handler() -> Json<AttendanceResponse> {
    let testattendance= AttendanceResponse {
      user: "user".to_string(),
      device_id: "device_id".to_string(),
      check_in_time: DateTime::from_timestamp_nanos(420420),
      check_out_time: Some(DateTime::from_timestamp_nanos(420420)), 
      duration: Some(1.0),
    };
    // TODO fill response with normal data
    
    Json(testattendance)
}