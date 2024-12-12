mod db;
mod models;
mod routes;

use std::net::{Ipv4Addr, SocketAddr};
use utoipa_axum::routes;
use axum::{extract::NestedPath, middleware, Router};
use std::io::Error;
use tokio::net::TcpListener;
use utoipa::{
    // openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    // Modify,
    OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use tower_http::validate_request::ValidateRequestHeaderLayer;

// macro definition to reduce repetitive code
macro_rules! lazy_env_var {
    ($name:ident, $env_var:expr, $default:expr) => {
        pub static $name: std::sync::LazyLock<String> =
            std::sync::LazyLock::new(|| match std::env::var($env_var) {
                Ok(value) if !value.is_empty() => value,
                _ => $default,
            });
    };
}

// lazy load env variables
lazy_env_var!(
    PYTHON_SERVICE_API_KEY,
    "PYTHON_SERVICE_API_KEY",
    panic!("PYTHON_SERVICE_API_KEY must be set and cannot be empty!")
);
lazy_env_var!(DB_USERNAME, "DB_USERNAME", "root".to_string());
lazy_env_var!(DB_PASSWORD, "DB_PASSWORD", "root".to_string());
lazy_env_var!(DATABASE_URL, "DATABASE_URL", "127.0.0.1:8000".to_string());
lazy_env_var!(JWT_SECRET, "JWT_SECRET", "superrandomdefaultsecret".to_string());

const YATTT_TAG: &str = "yatt";

pub const API_VERSION: &str = "v1";
const APPLICATION_PORT: u16 = 8080;

const DOCS_ROOT_ROUTE: &str = "/docs";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = YATTT_TAG, description = "Yet Another Time Tracking Tool API")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // load the env variables from .env
    dotenvy::dotenv().ok();

    // set up application logger
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

        db::surrealdb::connect(&DATABASE_URL).await.expect("Failed to connect to SurrealDB");


    let (unprotected_root_router, mut root_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(crate::routes::root::root_handler))
        .split_for_parts();

    let (protected_auth_router, auth_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(crate::routes::auth::auth_token_handler))
        .layer(middleware::from_fn(crate::routes::auth::authorization_layer))
        .routes(routes!(crate::routes::auth::auth_login_handler))
        .routes(routes!(crate::routes::auth::auth_register_handler))
        .split_for_parts();

    let (protected_card_router, card_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(crate::routes::card::card_create_handler))
        .routes(routes!(crate::routes::card::card_retrieve_handler))
        .routes(routes!(crate::routes::card::card_modify_handler))
        .routes(routes!(crate::routes::card::card_delete_handler))
        .layer(middleware::from_fn(crate::routes::auth::authorization_layer))
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
    let v1_routes = axum::Router::new()
        .merge(merged_router)
        .merge(Scalar::with_url(DOCS_ROOT_ROUTE, root_api));

    // define the `/api` router and nest `/v1` under `/api`
    let api_version_routes = axum::Router::new().nest("/v1", v1_routes);

    let app = axum::Router::new().nest("/api", api_version_routes);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, APPLICATION_PORT));
    let listener = TcpListener::bind(&address).await?;

    let api_adress = listener.local_addr()?;
    tracing::info!("API running on address http://{api_adress}/api/{API_VERSION}",);
    tracing::info!(
        "API-Docs running on address http://{api_adress}/api/{API_VERSION}{DOCS_ROOT_ROUTE}"
    );

    axum::serve(listener, app.into_make_service()).await
}
