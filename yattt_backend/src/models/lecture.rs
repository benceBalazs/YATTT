use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lecture {
    pub id: Option<String>, // SurrealDB record ID
    pub lv_name: String,    // Lecture name
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration: f64,      // Duration in minutes
    pub device_id: String,
}