use axum::{
  Json,
  extract::{Path},
};
use hyper::StatusCode;
use serde::{Serialize,Deserialize};
use crate::models::card::{Card, CardRequest};

#[derive(Serialize,Deserialize)]
pub struct CardRetrieveHandlerResponse {
    cards: Vec<CardRequest>
}

// TODO documentation
pub async fn card_create_handler(Json(payload): Json<CardRequest>) -> Json<StatusCode> {
    let mut response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO handle card creation and set appropriate response

    Json(response)
}

// TODO documentation
pub async fn card_retrieve_handler() -> Json<CardRetrieveHandlerResponse> {
    let testcard: CardRequest = CardRequest {
      tag_id: "1234".to_string(),
      name: "Default".to_string(), 
    };
    let response: CardRetrieveHandlerResponse = CardRetrieveHandlerResponse { cards: vec![testcard] };
    // TODO fill response with normal data & get user info via axum extractor (see wiki)

    Json(response)
}

// TODO documentation
pub async fn card_modify_handler(Path(CardRequest { tag_id, name }): Path<CardRequest>) -> Json<StatusCode> {
    let mut response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO modify requested card & return status
    
    Json(response)
}

// TODO documentation
pub async fn card_delete_handler(Path(CardRequest { tag_id, name }): Path<CardRequest>) -> Json<StatusCode> {
    let mut response: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
    // TODO delete requested card & return status

    Json(response)
}