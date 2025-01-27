use crate::{db, db::repositories::CardRepository, error::AppError, models::card::{Card, CardRequest}};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::jwt::Claims;

#[derive(Serialize, Deserialize)]
pub struct CardRetrieveHandlerResponse {
    cards: Vec<Card>,
}

#[utoipa::path(
    post,
    path = "/cards",
    params(
        CardRequest
    ),
    responses(
        (status = 201, description = "Successful creation of card by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub async fn card_create_handler(
    State(state): State<crate::YatttAppState>,
    Extension(user_data): Extension<Claims>,
    Json(payload): Json<CardRequest>,
) -> Result<(StatusCode, Json<Card>), AppError> {
    let user_id = user_data.user_id;

    let response = state
        .db
        .create_card(Card {
            id: None,
            user_id: Thing::from((db::db_constants::TABLE_USER.to_string(), user_id)),
            tag_id: payload.tag_id,
            card_name: payload.card_name,
        })
        .await?;

    let Some(response) = response else {
        return Err(AppError::InternalServerError);
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[utoipa::path(
    get,
    path = "/cards",
    responses(
        (status = 200, description = "Successful retrieval of card by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub async fn card_retrieve_handler(
    State(state): State<crate::YatttAppState>,
    Extension(user_data): Extension<Claims>,
) -> Result<Json<CardRetrieveHandlerResponse>,AppError> {
    let user_id = user_data.user_id;

    let response = state.db.get_cards(&user_id).await?;

    Ok(Json(CardRetrieveHandlerResponse { cards: response }))
}

#[utoipa::path(
    put,
    path = "/cards/{card_id}",
    params(
        CardRequest
    ),
    responses(
        (status = 200, description = "Successful modification of card by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub async fn card_modify_handler(
    State(state): State<crate::YatttAppState>,
    Extension(user_data): Extension<Claims>,
    Path(card_id): Path<String>,
    Json(payload): Json<CardRequest>
) -> Result<(StatusCode, Json<Card>), crate::error::AppError > {

    let user_id = user_data.user_id;

    let response = state.db.update_card(&card_id, Card {
        id: None,
        user_id: Thing::from((db::db_constants::TABLE_USER.to_string(), user_id.clone())),
        tag_id: payload.tag_id,
        card_name: payload.card_name,
    },&user_id).await?;


    let Some(response) = response else {
        return Err(AppError::InternalServerError);
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    delete,
    path = "/cards/{card_id}",
    responses(
        (status = 200, description = "Successful deletion of card by user"),
        (status = 400, description = "Bad Request, User sent malformed request"),
        (status = 401, description = "Unauthorized, User not authorized to use this route"),
        (status = 500, description = "Internal Server Error, Something went wrong on the APIs side")
    ),
    security(
        ("token_jwt" = [])
    )
)]
pub async fn card_delete_handler(
    State(state): State<crate::YatttAppState>,
    Extension(user_data): Extension<Claims>,
    Path(card_id): Path<String>,
) -> Result<StatusCode, crate::error::AppError> {

    let user_id = user_data.user_id;

    state.db.delete_card(&card_id, &user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}