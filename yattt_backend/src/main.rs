use std::net::{Ipv4Addr, SocketAddr};

use std::io::Error;
use tokio::net::TcpListener;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

// The API key is loaded from the environment variable PYTHON_SERVICE_API_KEY
static PYTHON_SERVICE_API_KEY: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    match std::env::var("PYTHON_SERVICE_API_KEY") {
        Ok(api_key) if !(api_key.is_empty()) => api_key,
        _ => panic!("PYTHON_SERVICE_API_KEY must be set and cannot be empty!"),
    }
});

const YATTT_TAG: &str = "yatt";
const API_VERSIONING: &str = "v5";
const APPLICATION_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Error> {

    // load the env variables from .env
    dotenvy::dotenv().ok();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = YATTT_TAG, description = "Yet Another Time Tracking Tool API")
        )
    )]
    struct ApiDoc;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(format!("/api/{API_VERSIONING}/").as_str(), yattt::router())
        .split_for_parts();

    let router = router.merge(Scalar::with_url("/scalar", api));

    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, APPLICATION_PORT));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await
}

mod yattt {
    use utoipa_axum::router::OpenApiRouter;

    pub(super) fn router() -> OpenApiRouter {
        OpenApiRouter::new()
    }
}
