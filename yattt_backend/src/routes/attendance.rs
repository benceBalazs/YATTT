use std::str::FromStr;
use crate::models::attendance::{Attendance, AttendanceRequest, AttendanceResponse};
use axum::{http, Extension, Json};
use axum::extract::{Request, State, Path};
use axum::http::HeaderMap;
use chrono::{Date, DateTime, Utc};
use hyper::StatusCode;
use surrealdb::sql::Kind::Datetime;
use crate::{db, PYTHON_SERVICE_API_KEY};
use crate::db::repositories::{AttendanceRepository, CardRepository, UserRepository};
use crate::error::AppError;
use crate::jwt::Claims;
use crate::models::card::Card;
use crate::models::lecture::Lecture;

#[derive(Clone)]
pub struct CurrentUser {
    pub username: String,
    pub password_hash: String,
}

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
) -> Result<(StatusCode), AppError> {

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

    let mut lecture_result = state.db.get_lectures_by_device_id(&payload.device_id).await?;

    if {lecture_result.is_empty()} {
        return Err(AppError::NotFound);
    }

    let mut response_attendance: Attendance;

    for lecture in lecture_result.iter() {

        let mut lecture_check_in_time = lecture.start_time;
        let mut lecture_check_out_time = lecture.end_time;

        let user_check_in_time = DateTime::<Utc>::from_str(&payload.check_in_time).unwrap();
        let user_check_out_time = DateTime::<Utc>::from_str(&payload.check_out_time).unwrap();

        if { user_check_out_time.lt(&lecture.end_time) } {
            lecture_check_out_time = user_check_out_time;
        }

        if { lecture.start_time.lt(&user_check_in_time)
            && lecture.end_time.gt(&user_check_in_time) } {
            lecture_check_in_time = user_check_in_time;

            let response_attendance_inner = state.db.create_attendance(Attendance {
                id: None,
                tag_id: payload.tag_id.clone(),
                user_id: user_thing.user_id.clone().unwrap(),
                device_id: payload.device_id.clone(),
                check_in_time: payload.check_in_time.clone(),
                check_out_time: payload.check_out_time.clone(),
                duration: payload.duration, // calc duration
            }).await?;

            let Some(_response_attendance_inner) = response_attendance_inner else {
                return Err(AppError::BadRequest)
            };

        } else {
            return Err(AppError::Generic("No Lecture at the given check in time frame".to_string()))
        }

    }

    Ok(StatusCode::OK)
}

#[utoipa::path(
    get,
    path = "/attendances",
    responses(
        (status = 201, description = "Successful retrieval of attendances by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub async fn attendance_retrieve_handler(
    State(state): State<crate::YatttAppState>,
    Extension(user_data): Extension<Claims>,
) -> Result<(StatusCode, Json<Vec<AttendanceResponse>>), AppError> {

    let user_id = user_data.user_id;

    let user_cards= state.db.get_cards(&user_id).await?;

    let mut user_attendances: Vec<AttendanceResponse> = Vec::new();

    let temp_attendance_result = state.db.get_attendances(&user_id).await?;

    if {temp_attendance_result.is_empty()} {
        return Ok((StatusCode::NO_CONTENT, Json(user_attendances)));
    }

    for user_attendance in temp_attendance_result.iter() {

        for user_card in user_cards.iter() {

            let mut lecture_result = state.db.get_lectures_by_device_id(&user_attendance.device_id).await?;

            if {lecture_result.is_empty()} {
                continue;
            }

            if  { user_card.tag_id.eq(&user_attendance.tag_id) } {

                for lecture in lecture_result.iter() {

                    let mut lecture_check_in_time = lecture.start_time;
                    let mut lecture_check_out_time = lecture.end_time;

                    let user_check_in_time = DateTime::<Utc>::from_str(&user_attendance.check_in_time).unwrap();
                    let user_check_out_time = DateTime::<Utc>::from_str(&user_attendance.check_out_time).unwrap();

                    if { user_check_out_time.lt(&lecture.end_time) } {
                        lecture_check_out_time = user_check_out_time;
                    }

                    if { lecture.start_time.lt(&user_check_in_time)
                        && lecture.end_time.gt(&user_check_in_time) } {
                        lecture_check_in_time = user_check_in_time;

                        print!("Looping {:?}", user_attendances);

                        user_attendances.push(AttendanceResponse {
                            card_name: user_card.card_name.to_string(),
                            lecture_name: lecture.lv_name.to_string(),
                            check_in_time: lecture_check_in_time.to_string(),
                            check_out_time: lecture_check_out_time.to_string(),
                            duration: user_attendance.duration,
                        })

                    }

                }
            }
        }
    }

    print!("{:?}", user_attendances);

    Ok((StatusCode::OK, Json(user_attendances)))
}
