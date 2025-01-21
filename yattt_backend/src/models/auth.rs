use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
}

