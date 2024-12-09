use std::net::{Ipv4Addr, SocketAddr};

use std::io::Error;
use tokio::net::TcpListener;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

const YATTT_TAG: &str = "yatt";
const API_VERSIONING: &str = "v5";

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 8080));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await
}

mod yattt {
    use utoipa_axum::router::OpenApiRouter;

    pub(super) fn router() -> OpenApiRouter {
        OpenApiRouter::new()
    }
}
