use axum::{
  body::Body,
  extract::{Json, Request}, 
  http::{self, Response, StatusCode},
  middleware::Next, 
  response::IntoResponse, Extension
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::models::user::User;

#[derive(Serialize)]
pub struct AuthApiResponse {
    access_token: String,
    token_type: String,
}

#[derive(Serialize, Deserialize)]
// Define a structure for holding claims data used in JWT tokens
pub struct Claims {
    pub exp: usize,
    pub iat: usize, 
    pub username: String,
}

// Define a structure for holding sign-in data
#[derive(Deserialize)]
pub struct SignInData {
    pub username: String,  // Email entered during sign-in
    pub password: String,  // Password entered during sign-in
}

pub struct AuthError {
  message: String,
  status_code: StatusCode,
}

impl IntoResponse for AuthError {
  fn into_response(self) -> Response<Body> {
      let body = Json(json!({
          "error": self.message,
      }));

      (self.status_code, body).into_response()
  }
}

pub async fn auth_token_handler(Extension(user_data): Extension<CurrentUser>) -> Result<Json<AuthApiResponse>, StatusCode> {
    // TODO decrypt token and generate new one, temporary code for testing purposes

    // Generate a JWT token for the authenticated user
    let token = encode_jwt(user_data.username)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle JWT encoding errors

    // Return the token as a JSON-wrapped string
    Ok(Json(AuthApiResponse { access_token: token, token_type: "Bearer".to_string() }))
}

pub async fn auth_login_handler(Json(user_data): Json<SignInData>) -> Result<Json<AuthApiResponse>, StatusCode> {
    // Attempt to retrieve user information based on the provided email
    let user = match retrieve_user_by_username(&user_data.username) {
      Some(user) => user,  // User found, proceed with authentication
      None => return Err(StatusCode::UNAUTHORIZED), // User not found, return unauthorized status
    };

    // Verify the password provided against the stored hash
    if !verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? // Handle bcrypt errors
    {
        return Err(StatusCode::UNAUTHORIZED); // Password verification failed, return unauthorized status
    }

    // Generate a JWT token for the authenticated user
    let token = encode_jwt(user.username)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?; // Handle JWT encoding errors

    // Return the token as a JSON-wrapped string
    Ok(Json(AuthApiResponse { access_token: token, token_type: "Bearer".to_string() }))
}

pub async fn auth_register_handler(Json(user): Json<User>) -> impl IntoResponse {
    let Some(created_user_id) = crate::db::surrealdb::create_user(user).await else {
        return (
            hyper::StatusCode::BAD_REQUEST,
            Json(json!({ "error": "User not created" })),
        );
    };

    // TODO return token info with id of created user instead
    // TODO hash PW using hash fn and save into DB

    (
        hyper::StatusCode::CREATED,
        Json(json!(created_user_id)),
    )
}

#[derive(Clone)]
pub struct CurrentUser {
    pub username: String,
    pub password_hash: String
}

// Function to simulate retrieving user data from a database based on email
fn retrieve_user_by_username(username: &str) -> Option<CurrentUser> {
    // TODO use DB request instead of hardcoded user
    let current_user: CurrentUser = CurrentUser {
        username: "samplename".to_string(),
        password_hash: "$2b$12$Gwf0uv3dsL7JLfo0CC/NCOoijK2vQ/wbgP.LeNup8vj6gg31IiFkm".to_string()
    };
    Some(current_user)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
  verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
  let hash = hash(password, DEFAULT_COST)?;
  Ok(hash)
}

pub fn encode_jwt(username: String) -> Result<String, StatusCode> {
  let now = Utc::now();
  let expire: chrono::TimeDelta = Duration::hours(24);
  let exp: usize = (now + expire).timestamp() as usize;
  let iat: usize = now.timestamp() as usize;
  let claim = Claims { iat, exp, username };

  encode(
      &Header::default(),
      &claim,
      &EncodingKey::from_secret(&crate::JWT_SECRET.as_bytes()),
  )
  .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, StatusCode> {
  let result: Result<TokenData<Claims>, StatusCode> = decode(
      &token,
      &DecodingKey::from_secret(&crate::JWT_SECRET.as_bytes()),
      &Validation::default(),
  )
  .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
  result
}

pub async fn authorization_layer(mut req: Request, next: Next) -> Result<Response<Body>, AuthError> {
  let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
  let auth_header = match auth_header {
      Some(header) => header.to_str().map_err(|_| AuthError {
          message: "Empty header is not allowed".to_string(),
          status_code: StatusCode::FORBIDDEN
      })?,
      None => return Err(AuthError {
          message: "Please add the JWT token to the header".to_string(),
          status_code: StatusCode::FORBIDDEN
      }),
  };
  let mut header = auth_header.split_whitespace();
  let (_bearer, token) = (header.next(), header.next());
  let token_data = match decode_jwt(token.unwrap().to_string()) {
      Ok(data) => data,
      Err(_) => return Err(AuthError {
          message: "Unable to decode token".to_string(),
          status_code: StatusCode::UNAUTHORIZED
      }),
  };
  // Fetch the user details from the database
  // TODO replace with wanted inject data for services after middleware
  let current_user = match retrieve_user_by_username(&token_data.claims.username) {
      Some(user) => user,
      None => return Err(AuthError {
          message: "You are not an authorized user".to_string(),
          status_code: StatusCode::UNAUTHORIZED
      }),
  };
  req.extensions_mut().insert(current_user);
  Ok(next.run(req).await)
}