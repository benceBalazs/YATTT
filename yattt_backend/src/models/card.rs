use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: Option<String>, // SurrealDB record ID
    pub user: String,       // Reference to User's ID
    pub tag_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardRequest {
    pub tag_id: String,
    pub name: String,
}