use crate::models::user::User;
use crate::jwt::{Claims, TokenEncoder};
use axum::{
    body::Body,
    extract::{Json, Request},
    http::{self, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
}

// Define a structure for holding sign-in data
#[derive(Deserialize, IntoParams, ToSchema)]
pub struct SignInData {
    pub username: String,
    pub password: String,
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

/// Search Todos by query params.
#[utoipa::path(
    get,
    path = "/auth",
    responses(
        (status = 200, description = "Successful re-authentication by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side - try later again")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub async fn auth_token_handler(
    Extension(user_data): Extension<TokenData<crate::jwt::Claims>>,
) -> Result<Json<TokenResponse>, StatusCode> {
    // Generate a JWT token for the authenticated user
    let token = crate::YatttEncoder::encode_jwt(user_data.claims.user_id)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?; // Handle JWT encoding errors

    // Return the token as a JSON-wrapped string
    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    params(
        SignInData
    ),
    responses(
        (status = 200, description = "Successful re-authentication by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side - try later again")
    )
)]
pub async fn auth_login_handler(
    Json(user_data): Json<SignInData>,
) -> Result<Json<TokenResponse>, StatusCode> {
    // Attempt to retrieve user information based on the provided user_data
    let db_result = crate::db::surrealdb::check_user(&user_data.username).await;

    // Handle database errors
    let Ok(found_user) = db_result else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    // User not found, return unauthorized status
    let Some(correct_user) = found_user else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Verify the password provided against the stored hash
    if !verify_password(&user_data.password, &correct_user.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    // Handle bcrypt errors
    {
        return Err(StatusCode::UNAUTHORIZED); // Password verification failed, return unauthorized status
    }

    // user entry has no key
    let Some(user_thing) = correct_user.id else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    // Generate a JWT token for the authenticated user
    let token =
        encode_jwt(user_thing.id.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return the token as a JSON-wrapped string
    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

#[utoipa::path(
    post,
    path = "/auth/register",
    params(
        SignInData
    ),
    responses(
        (status = 200, description = "Successful re-authentication by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side - try later again")
    )
)]
pub async fn auth_register_handler(
    Json(mut user): Json<SignInData>,
) -> Result<Json<TokenResponse>, StatusCode> {
    // hash password
    user.password = hash_password(&user.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Attempt to create the user
    let db_result = crate::db::surrealdb::create_user(user).await;

    // Handle database errors
    let Ok(created_user_maybe) = db_result else {
        tracing::error!("Error creating user: {:?}", db_result.unwrap_err());
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    // User not found, return unauthorized status
    let Some(created_user) = created_user_maybe else {
        tracing::error!("User not created: {:?}", created_user_maybe);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    dbg!(&created_user);

    // user entry has no key
    let Some(user_thing) = created_user.id else {
        tracing::error!("User Entry {:?} has no key", created_user);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    // Generate a JWT token for the authenticated user
    let token = crate::YatttEncoder::encode_jwt(user_thing.id.to_string())
        .ok_or(AppError::InternalServerError)?;

    // Return the token as a JSON-wrapped string
    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

    Ok((
        StatusCode::CREATED,
        Json(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
        }),
    ))
}

pub async fn authorization_layer(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::BAD_REQUEST,
        })?,
        None => {
            return Err(AuthError {
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::BAD_REQUEST,
            })
        }
    };
    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());
    let token_data =
        crate::YatttEncoder::decode_jwt(token.unwrap().to_string()).ok_or(AuthError {
            message: "Unable to decode token".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        })?;

    // Attempt to retrieve user information based on the provided user_data
    let db_result = crate::db::surrealdb::check_user_by_id(&token_data.claims.user_id).await;

    // Handle database errors
    let Ok(found_user) = db_result else {
        return Err(AuthError {
            message: "Error fetching user".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        });
    };

    // User not found, return unauthorized status
    if found_user.is_none() {
        return Err(AuthError {
            message: "User does not exist".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    };

    req.extensions_mut().insert(token_data.clone());
    Ok(next.run(req).await)
}
