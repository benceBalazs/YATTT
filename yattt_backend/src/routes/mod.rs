pub mod attendance;
pub mod auth;
pub mod card;
pub mod root;

use axum::{middleware, Extension, Router};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_scalar::{Scalar, Servable as ScalarServable};


#[derive(utoipa::OpenApi)]
#[openapi(
    tags(
        (name = crate::YATTT_TAG, description = "Yet Another Time Tracking Tool API")
    )
)]
pub struct ApiDoc;

pub fn setup_routes(state: crate::YatttAppState) -> Router {
    let (unprotected_root_router, mut root_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(crate::routes::root::root_handler))
        .split_for_parts();

    let (protected_auth_router, auth_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(crate::routes::auth::auth_token_handler))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::routes::auth::authorization_layer,
        ))
        .routes(routes!(crate::routes::auth::auth_login_handler))
        .routes(routes!(crate::routes::auth::auth_register_handler))
        .split_for_parts();

    let (protected_card_router, card_api) =
        OpenApiRouter::<crate::YatttAppState>::with_openapi(ApiDoc::openapi())
            .routes(routes!(crate::routes::card::card_create_handler))
            .routes(routes!(crate::routes::card::card_retrieve_handler))
            .routes(routes!(crate::routes::card::card_modify_handler))
            .routes(routes!(crate::routes::card::card_delete_handler))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                crate::routes::auth::authorization_layer,
            ))
            .split_for_parts();

    let merged_router = axum::Router::new()
        .merge(unprotected_root_router)
        .merge(protected_auth_router)
        .merge(protected_card_router);

    root_api.merge(auth_api);
    root_api.merge(card_api);

    // TODO uncomment after adding annotations in attendance.rs
    // let attendance_router_technical_protected: OpenApiRouter = OpenApiRouter::new()
    //     .routes(routes!(crate::routes::attendance::attendance_create_handler))
    //     .layer(ValidateRequestHeaderLayer::bearer(&PYTHON_SERVICE_API_KEY));

    // let attendace_router_user_protected: OpenApiRouter = OpenApiRouter::new()
    //     .routes(routes!(crate::routes::attendance::attendance_retrieve_handler))
    //     .layer(middleware::from_fn(crate::routes::auth::authorization_layer));

    // define the `/v1` router
    let v1_routes =
        axum::Router::new()
            .merge(merged_router)
            .merge(utoipa_scalar::Scalar::with_url(
                crate::DOCS_ROOT_ROUTE,
                root_api,
            ));

    // define the `/api` router and nest `/v1` under `/api`
    let api_version_routes = axum::Router::new().nest("/v1", v1_routes).with_state(state);

    axum::Router::new().nest("/api", api_version_routes)
}
