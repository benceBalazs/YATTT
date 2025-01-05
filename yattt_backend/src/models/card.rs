use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: Option<String>, // SurrealDB record ID
    pub user: String,       // Reference to User's ID
    pub tag_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CardRequest {
    pub tag_id: String,
    pub name: String,
}