use crate::encryption::PasswordEncrypter;
use crate::error::AppError;
use crate::jwt::{Claims, TokenEncoder};
use crate::models::auth::TokenResponse;
use crate::db::repositories::UserRepository;
use crate::{YatttAppState, YatttEncrypter};
use axum::{
    body::Body,
    extract::{Json, Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension,
};
use serde::Deserialize;
use serde_json::json;
use utoipa::{IntoParams, ToSchema};

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
    Extension(user_data): Extension<Claims>,
) -> Result<Json<TokenResponse>, StatusCode> {
    // Generate a JWT token for the authenticated user

    let token = crate::YatttEncoder::encode_jwt(user_data.user_id)
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
    State(state): State<YatttAppState>,
    Json(user_data): Json<SignInData>,
) -> Result<Json<TokenResponse>, AppError> {
    // Attempt to retrieve user information based on the provided user_data
    let found_user = state
        .db
        .get_by_username(&user_data.username)
        .await
        .map_err(AppError::from)?;

    // User not found, return unauthorized status
    let Some(correct_user) = found_user else {
        return Err(AppError::Unauthorized);
    };

    // Verify the password provided against the stored hash

    // Handle bcrypt errors
    if !crate::encryption::BcryptPasswordEncrypter::verify_password(
        &user_data.password,
        &correct_user.password,
    ) {
        return Err(AppError::Unauthorized); // Password verification failed, return unauthorized status
    }

    // user entry has no key
    let Some(user_thing) = correct_user.id else {
        return Err(AppError::InternalServerError);
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
        (status = 415, description = "Unsupported Media Type, User sent malformed request"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side - try later again")
    )
)]
pub async fn auth_register_handler(
    State(state): State<YatttAppState>,
    Json(mut user): Json<SignInData>,
) -> Result<(StatusCode, Json<TokenResponse>), AppError> {
    // Verify the input length 
    if user.password.len() < 8 || user.username.len() < 3 {
        return Err(AppError::BadRequest);
    };

    // Hash the password
    user.password =
        YatttEncrypter::hash_password(&user.password).ok_or(AppError::InternalServerError)?;

    // Attempt to create the user
    let possible_user = state.db.create_user(user).await.map_err(AppError::from)?;

    // User not found, return unauthorized status
    let Some(created_user) = possible_user else {
        tracing::error!("User not created: {:?}", possible_user);
        return Err(AppError::InternalServerError);
    };

    // user entry has no key
    let Some(user_thing) = created_user.id else {
        tracing::error!("User Entry {:?} has no key", created_user);
        return Err(AppError::InternalServerError);
    };

    // Generate a JWT token for the authenticated user
    let token = crate::YatttEncoder::encode_jwt(user_thing.id.to_string())
        .ok_or(AppError::InternalServerError)?;

    // Return the token as a JSON-wrapped string
    Ok((
        StatusCode::CREATED,
        Json(TokenResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
        }),
    ))
}

pub async fn authorization_layer(
    State(state): State<crate::YatttAppState>,
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

    println!("{:?}", token_data);

    let token_injector = token_data.clone();

    // Attempt to retrieve user information based on the provided user_data
    let db_result = state.db.get_by_id(&token_data.user_id).await;

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

    req.extensions_mut().insert(token_injector);

    Ok(next.run(req).await)
}
