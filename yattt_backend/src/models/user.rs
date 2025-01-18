// use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
// use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>, // SurrealDB will use a record ID automatically
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdExtractor {
    pub user_id: Option<Thing>,
}