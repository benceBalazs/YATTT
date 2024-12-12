use crate::models::attendance::{Attendance, AttendanceRequest};
use axum::{Extension, Json};
use chrono::DateTime;
use hyper::StatusCode;

#[derive(Clone)]
pub struct CurrentUser {
    pub username: String,
    pub password_hash: String,
}

// TODO documentation
pub async fn attendance_create_handler(
    Json(attendance_data): Json<AttendanceRequest>,
) -> (StatusCode, String) {
    // // Attempt to retrieve user information based on the provided user_data
    // let db_result = crate::db::surrealdb::get_user_by_tag_id(&attendance_data.tag_id).await;

    // // Handle database errors
    // let Ok(found_user) = db_result else {
    //     return Err(StatusCode::INTERNAL_SERVER_ERROR);
    // };

    // // User not found, return unauthorized status
    // let Some(correct_user) = found_user else {
    //     return Err(StatusCode::UNAUTHORIZED);
    // };

    // // create attendance
    // let actual_attendance = Attendance {
    //     user: attendance_data.user,
    //     device_id: attendance_data.device_id,
    //     check_in_time: attendance_data.check_in_time,
    //     check_out_time: attendance_data.check_out_time,
    //     duration: attendance_data.duration
    // }

    let response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO create attendance by utilizing payload

    (response, "success".to_string())
}

// TODO documentation
pub async fn attendance_retrieve_handler(
    Extension(user_data): Extension<CurrentUser>,
) -> Json<Attendance> {
    let testattendance = Attendance {
        id: None,
        user: user_data.username.to_string(),
        device_id: "device_id".to_string(),
        check_in_time: DateTime::from_timestamp_nanos(420420),
        check_out_time: DateTime::from_timestamp_nanos(420420),
        duration: 1.0,
    };
    // TODO fill response with normal data

    Json(testattendance)
}
