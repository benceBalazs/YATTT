use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;
use utoipa::{IntoParams, IntoResponses, ToSchema};


#[derive(Debug,PartialEq, Serialize, Deserialize)]
pub struct Attendance {
    pub id: Option<Thing>, // SurrealDB will generate a record ID
    pub tag_id: String,
    pub user_id: Thing,       // Reference to User's ID
    pub device_id: String,
    pub check_in_time: String,
    pub check_out_time: String,
    pub duration: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams, Clone)]
pub struct AttendanceRequest {
    pub tag_id: String,
    pub device_id: String,
    pub check_in_time: String,
    pub check_out_time: String,
    pub duration: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct AttendanceResponse {
    pub card_name: String,
    pub lecture_name: String,
    pub check_in_time: String,
    pub check_out_time: String,
    pub duration: f64,
}