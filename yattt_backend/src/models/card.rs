use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: Option<Thing>, // SurrealDB record ID
    pub user_id: Thing,       // Reference to User's ID
    pub tag_id: String,
    pub card_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, IntoParams, ToSchema)]
pub struct CardRequest {
    pub tag_id: String,
    pub card_name: String,
}