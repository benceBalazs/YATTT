use crate::models::attendance::{Attendance, AttendanceRequest};
use axum::{http, Extension, Json};
use axum::extract::{Request, State, Path};
use axum::http::HeaderMap;
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use crate::{db, PYTHON_SERVICE_API_KEY};
use crate::db::repositories::{AttendanceRepository, UserRepository};
use crate::error::AppError;

#[derive(Clone)]
pub struct CurrentUser {
    pub username: String,
    pub password_hash: String,
}
#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/attendances",
    responses(
        (status = 201, description = "Successful creation of attendances by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side")
    )
)]
pub async fn attendance_create_handler(
    State(state): State<crate::YatttAppState>,
    headers: HeaderMap,
    Json(payload): Json<AttendanceRequest>,
) -> Result<(StatusCode, Json<Attendance>), AppError> {

    let Some(auth_header) = headers.get("Authorization") else {
        return Err(AppError::BadRequest)
    };

    let Ok(auth_value) = auth_header.to_str() else {
        return Err(AppError::NotFound)
    };

    let mut header = auth_value.split_whitespace();
    let (_bearer, api_key) = (header.next(), header.next());

    let Some(api_key) = api_key else {
            return Err(AppError::BadRequest)
    };

    if api_key != PYTHON_SERVICE_API_KEY.to_string() {
        return Err(AppError::Unauthorized)
    };

    let response_username = state.db.get_by_tag_id(&payload.tag_id).await?;

    let Some(user_thing) = response_username else {
        return Err(AppError::NotFound)
    };

    let response_attendance = state.db.create_attendance(Attendance {
        id: None,
        user_id: user_thing.user_id.unwrap(),
        device_id: payload.device_id,
        check_in_time: payload.check_in_time,
        check_out_time: payload.check_out_time,
        duration: payload.duration,
    }).await?;

    let Some(attendance) = response_attendance else {
            return Err(AppError::InternalServerError)
    };

    Ok((StatusCode::CREATED, Json(attendance)))
}

#[utoipa::path(
    get,
    path = "/attendances",
    responses(
        (status = 201, description = "Successful creation of attendances by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side")
    )
)]
pub async fn attendance_retrieve_handler(
    Extension(user_data): Extension<CurrentUser>,
) -> Result<Json<Attendance>, AppError> {
    let testattendance = Attendance {
        id: None,
        user_id: "user_data.id".to_string().parse().unwrap(),
        device_id: "device_id".to_string(),
        check_in_time: DateTime::from_timestamp_nanos(420420).to_string(),
        check_out_time: DateTime::from_timestamp_nanos(420420).to_string(),
        duration: 1.0,
    };
    // TODO fill response with normal data

    Ok(Json(testattendance))
}
