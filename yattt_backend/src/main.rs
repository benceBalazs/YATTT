mod routes;

use std::net::{Ipv4Addr, SocketAddr};

use std::io::Error;
use tokio::net::TcpListener;
use utoipa::{
    // openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    // Modify,
    OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

// The API key is loaded from the environment variable PYTHON_SERVICE_API_KEY
#[allow(dead_code)]
static PYTHON_SERVICE_API_KEY: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| match std::env::var("PYTHON_SERVICE_API_KEY") {
        Ok(api_key) if !(api_key.is_empty()) => api_key,
        _ => panic!("PYTHON_SERVICE_API_KEY must be set and cannot be empty!"),
    });

const YATTT_TAG: &str = "yatt";
pub const API_VERSION: &str = "v1";
const APPLICATION_PORT: u16 = 8080;
const DOCS_ROUTE: &str = "/docs";

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

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = YATTT_TAG, description = "Yet Another Time Tracking Tool API")
        )
    )]
    struct ApiDoc;

    // generate the documentation router
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(DOCS_ROUTE, yattt::router())
        .split_for_parts();

    // define the `/v1` router
    let v1_routes = axum::Router::new()
        .route("/", axum::routing::get(crate::routes::root::root_handler))
        .merge(router)
        .merge(Scalar::with_url(DOCS_ROUTE, api));

    // define the `/api` router and nest `/v1` under `/api`
    let api_version_routes = axum::Router::new().nest("/v1", v1_routes);

    let app = axum::Router::new().nest("/api", api_version_routes);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, APPLICATION_PORT));
    let listener = TcpListener::bind(&address).await?;

    let api_adress = listener.local_addr()?;
    tracing::info!("API running on address http://{api_adress}/api/{API_VERSION}",);
    tracing::info!("API-Docs running on address http://{api_adress}/api/{API_VERSION}{DOCS_ROUTE}");

    axum::serve(listener, app.into_make_service()).await
}

mod yattt {
    use utoipa_axum::router::OpenApiRouter;

    pub(super) fn router() -> OpenApiRouter {
        OpenApiRouter::new()
    }
}
