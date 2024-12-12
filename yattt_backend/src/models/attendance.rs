use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use surrealdb::sql::Thing; // Use chrono for handling datetime

#[derive(Debug, Serialize, Deserialize)]
pub struct Attendance {
    pub id: Option<Thing>, // SurrealDB will generate a record ID
    pub user: String,       // Reference to User's ID
    pub device_id: String,
    pub check_in_time: DateTime<Utc>,
    pub check_out_time: DateTime<Utc>,
    pub duration: f64,
}

pub struct AttendanceRequest {
    pub tag_id: String,
    pub device_id: String,
    pub check_in_time: DateTime<Utc>,
    pub check_out_time: DateTime<Utc>,
    pub duration: f64,
}