use axum::{
  Json,
  extract::Path,
};
use hyper::StatusCode;
use serde::{Serialize,Deserialize};
use crate::models::card::CardRequest;

#[derive(Serialize,Deserialize)]
pub struct CardRetrieveHandlerResponse {
    cards: Vec<CardRequest>
}

#[utoipa::path(
    post,
    path = "/cards",
    params(
        CardRequest
    ),
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
pub async fn card_create_handler(Json(payload): Json<CardRequest>) -> (StatusCode, String) {
    let response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO handle card creation and set appropriate response

    (response, "success".to_string())
}

#[utoipa::path(
    get,
    path = "/cards",
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
pub async fn card_retrieve_handler() -> Json<CardRetrieveHandlerResponse> {
    let testcard: CardRequest = CardRequest {
      tag_id: "1234".to_string(),
      name: "Default".to_string(), 
    };
    let response: CardRetrieveHandlerResponse = CardRetrieveHandlerResponse { cards: vec![testcard] };
    // TODO fill response with normal data & get user info via axum extractor (see wiki)
    
    Json(response)
}

#[utoipa::path(
    put,
    path = "/cards/{tag_id}",
    params(
        CardRequest
    ),
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
pub async fn card_modify_handler(Path(CardRequest { tag_id, name }): Path<CardRequest>) -> (StatusCode, String) {
    let response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO modify requested card & return status
    
    (response, "success".to_string())
}

#[utoipa::path(
    delete,
    path = "/cards/{tag_id}",
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
pub async fn card_delete_handler(Path(CardRequest { tag_id, name }): Path<CardRequest>) -> (StatusCode, String) {
    let response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO delete requested card & return status

    (response, "success".to_string())
}