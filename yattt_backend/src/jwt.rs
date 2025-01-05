

#[derive(Clone, serde::Serialize, serde::Deserialize)]
// Define a structure for holding claims data used in JWT tokens
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub user_id: String,
}

pub trait TokenEncoder {
    fn encode_jwt(user_id: String) -> Option<String>;
    fn decode_jwt(token: String) -> Option<Claims>;
}

pub struct JWTEncoder {}

use chrono::{Duration, Utc};

impl TokenEncoder for JWTEncoder {
    fn encode_jwt(user_id: String) -> Option<String> {
        let now = Utc::now();
        let expire: chrono::TimeDelta = Duration::hours(24);
        let exp: usize = (now + expire).timestamp() as usize;
        let iat: usize = now.timestamp() as usize;
        let claim = Claims { iat, exp, user_id };
    
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &jsonwebtoken::EncodingKey::from_secret(crate::JWT_SECRET.as_bytes()),
        )
        .ok()
    }
    
    fn decode_jwt(token: String) -> Option<Claims> {
        let result = jsonwebtoken::decode(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(crate::JWT_SECRET.as_bytes()),
            &jsonwebtoken::Validation::default(),
        ).ok()?.claims;
        result
    }
}


