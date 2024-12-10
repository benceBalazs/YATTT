use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc}; // Use chrono for handling datetime

#[derive(Debug, Serialize, Deserialize)]
pub struct Attendance {
    pub id: Option<String>, // SurrealDB will generate a record ID
    pub user: String,       // Reference to User's ID
    pub device_id: String,
    pub check_in_time: DateTime<Utc>,
    pub check_out_time: Option<DateTime<Utc>>, // Optional field for check-out time
    pub duration: Option<f64>, // Duration in hours (optional)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceResponse {
    pub user: String,       // Reference to User's ID
    pub device_id: String,
    pub check_in_time: DateTime<Utc>,
    pub check_out_time: Option<DateTime<Utc>>, // Optional field for check-out time
    pub duration: Option<f64>, // Duration in hours (optional)
}