use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>, // SurrealDB will use a record ID automatically
    pub username: String,
    pub password: String,
}