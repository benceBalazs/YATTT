mod db;
mod models;
mod routes;

use std::net::{Ipv4Addr, SocketAddr};

use axum::{middleware, Router};
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

const AUTH_TOKEN_ROUTE: &str = "/auth";
const AUTH_LOGIN_ROUTE: &str = "/auth/login";
const AUTH_REGISTER_ROUTE: &str = "/auth/register";

const CARD_CREATE_ROUTE: &str = "/cards";
const CARD_KEYED_ROUTE: &str = "/cards/:id";

const ATTENDANCE_CREATE_ROUTE: &str = "/attendances";

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

    // generate the documentation router
    let (docs_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(DOCS_ROOT_ROUTE, yattt::router())
        .split_for_parts();

    // generate auth routes
    let auth_routes: Router = axum::Router::new()
        .route(
            AUTH_TOKEN_ROUTE,
            axum::routing::get(crate::routes::auth::auth_token_handler),
        )
        .layer(middleware::from_fn(crate::routes::auth::authorization_layer))
        .route(
            AUTH_LOGIN_ROUTE,
            axum::routing::post(crate::routes::auth::auth_login_handler),
        )
        .route(
            AUTH_REGISTER_ROUTE,
            axum::routing::post(crate::routes::auth::auth_register_handler),
        );

    // generate auth router by merging all auth routes
    let auth_router: Router = axum::Router::new().merge(auth_routes);

    // generate auth routes
    let card_routes: Router = axum::Router::new()
        .route(
            CARD_CREATE_ROUTE,
            axum::routing::post(crate::routes::card::card_create_handler),
        )
        .route(
            CARD_CREATE_ROUTE,
            axum::routing::get(crate::routes::card::card_retrieve_handler),
        )
        .route(
            CARD_KEYED_ROUTE,
            axum::routing::put(crate::routes::card::card_modify_handler),
        )
        .route(
            CARD_KEYED_ROUTE,
            axum::routing::delete(crate::routes::card::card_delete_handler),
        )
        .layer(middleware::from_fn(crate::routes::auth::authorization_layer));

    // generate auth router by merging all auth routes
    let card_router: Router = axum::Router::new().merge(card_routes);

    // generate auth routes
    let attendance_routes_technical: Router = axum::Router::new()
        .route(ATTENDANCE_CREATE_ROUTE, axum::routing::post(crate::routes::attendance::attendance_create_handler))
        .layer(ValidateRequestHeaderLayer::bearer(&PYTHON_SERVICE_API_KEY));

    // generate auth routes
    let attendance_routes_user: Router = axum::Router::new()
        .route(ATTENDANCE_CREATE_ROUTE, axum::routing::get(crate::routes::attendance::attendance_retrieve_handler))
        .layer(middleware::from_fn(crate::routes::auth::authorization_layer));


    // generate auth router by merging all auth routes
    let attendance_router: Router = axum::Router::new()
        .merge(attendance_routes_technical)
        .merge(attendance_routes_user);

    // define the `/v1` router
    let v1_routes = axum::Router::new()
        .route("/", axum::routing::get(crate::routes::root::root_handler))
        .merge(auth_router)
        .merge(card_router)
        .merge(attendance_router)
        .merge(docs_router)
        .merge(Scalar::with_url(DOCS_ROOT_ROUTE, api));

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

mod yattt {
    use utoipa_axum::router::OpenApiRouter;

    pub(super) fn router() -> OpenApiRouter {
        OpenApiRouter::new()
    }
}
