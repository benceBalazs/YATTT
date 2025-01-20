use crate::db::repositories::{AttendanceRepository, CardRepository, UserRepository};
use crate::error::AppError;
use crate::jwt::Claims;
use crate::models::attendance::{Attendance, AttendanceRequest, AttendanceResponse};
use crate::models::lecture::Lecture;
use crate::{PYTHON_SERVICE_API_KEY};
use axum::extract::{State};
use axum::http::HeaderMap;
use axum::{Extension, Json};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use std::str::FromStr;

static SECONDS_IN_AN_HOUR: f64 = 3600.0;

#[derive(Clone)]
pub struct CurrentUser {
    pub username: String,
    pub password_hash: String,
}

fn get_attended_lecture(
    user_check_in_time: DateTime<Utc>,
    user_check_out_time: DateTime<Utc>,
    lectures: &[Lecture], //lectures with correct device_id
) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    // user checkin should be after lecture check in
    // user check-in should be before lecture check-out
    let valid_lectures = lectures
        .iter()
        .filter(|lecture| {
            lecture.start_time.lt(&user_check_in_time) && lecture.end_time.gt(&user_check_in_time)
        })
        .collect::<Vec<_>>();

    // a lecture should be found
    let Some(lecture) = valid_lectures.first() else {
        return None;
    };

    // when user checkout earlier than lecture end => use checkout time
    let end_time = if user_check_out_time.lt(&lecture.end_time) {
        user_check_out_time
    } else {
        lecture.end_time
    };

    Some((lecture.start_time, end_time))
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
) -> Result<(StatusCode, Json<Attendance>), AppError> {
    // auth header exists
    let Some(auth_header) = headers.get("Authorization") else {
        return Err(AppError::BadRequest);
    };

    // header value exists
    let Ok(auth_value) = auth_header.to_str() else {
        return Err(AppError::NotFound);
    };

    // split header
    let mut header = auth_value.split_whitespace();
    let (_bearer, api_key) = (header.next(), header.next());

    // api key exists
    let Some(api_key) = api_key else {
        return Err(AppError::BadRequest);
    };

    // api key matches
    if api_key != PYTHON_SERVICE_API_KEY.to_string() {
        return Err(AppError::Unauthorized);
    };

    let Ok(user_check_in_time) = DateTime::<Utc>::from_str(&payload.check_in_time) else {
        return Err(AppError::BadRequest);
    };

    let Ok(user_check_out_time) = DateTime::<Utc>::from_str(&payload.check_out_time) else {
        return Err(AppError::BadRequest);
    };

    // user exists
    let Some(user_thing) = state.db.get_by_tag_id(&payload.tag_id).await? else {
        return Err(AppError::NotFound);
    };

    let Some(user_id) = user_thing.user_id else {
        return Err(AppError::Unauthorized);
    };

    // get lectures by device_id
    let lectures = state
        .db
        .get_lectures_by_device_id(&payload.device_id)
        .await?;

    // make sure lectures exist
    if lectures.is_empty() {
        return Err(AppError::NotFound);
    }

    // check if user has attended a lecture
    let Some(attendance) = get_attended_lecture(user_check_in_time, user_check_out_time, &lectures)
    else {
        return Err(AppError::Generic(
            "No Lecture at the given check-in time frame".to_string(),
        ));
    };

    // create attendance entry
    let Some(response_attendance) = state
        .db
        .create_attendance(Attendance {
            id: None,
            tag_id: payload.tag_id,
            user_id,
            device_id: payload.device_id,
            check_in_time: attendance.0.to_rfc3339(),
            check_out_time: attendance.1.to_rfc3339(),
            duration: ((attendance.1 - attendance.0).num_seconds() as f64) / SECONDS_IN_AN_HOUR,
        })
        .await?
    else {
        return Err(AppError::InternalServerError);
    };

    Ok((StatusCode::CREATED, Json(response_attendance)))
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

    let user_cards = state.db.get_cards(&user_id).await?;

    let mut user_attendances: Vec<AttendanceResponse> = Vec::new();

    let temp_attendance_result = state.db.get_attendances(&user_id).await?;

    if temp_attendance_result.is_empty() {
        return Ok((StatusCode::NOT_FOUND, Json(user_attendances)));
    }

    for user_attendance in temp_attendance_result.iter() {
        for user_card in user_cards.iter() {
            let lecture_result = state
                .db
                .get_lectures_by_device_id(&user_attendance.device_id)
                .await?;

            if lecture_result.is_empty() {
                continue;
            }

            if user_card.tag_id.eq(&user_attendance.tag_id) {
                for lecture in lecture_result.iter() {
                    let user_check_in_time =
                        DateTime::<Utc>::from_str(&user_attendance.check_in_time).unwrap();
                    let user_check_out_time =
                        DateTime::<Utc>::from_str(&user_attendance.check_out_time).unwrap();

                    user_attendances.push(AttendanceResponse {
                        card_name: user_card.card_name.to_string(),
                        lecture_name: lecture.lv_name.to_string(),
                        check_in_time: user_check_in_time.to_rfc3339(),
                        check_out_time: user_check_out_time.to_rfc3339(),
                        duration: user_attendance.duration,
                    })
                }
            }
        }
    }

    print!("{:?}", user_attendances);

    Ok((StatusCode::OK, Json(user_attendances)))
}