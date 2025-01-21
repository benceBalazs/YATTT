// #[cfg(feature = "test")]
mod test_modules;

use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};
use surrealdb::engine::local::Mem;
use surrealdb::opt::auth::Root as DatabaseCredentials;
use tokio::net::TcpListener;
use yattt_backend::db::surrealdb::SurrealDbBackend;
use yattt_backend::{ db, AppState};

fn register_logger() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn run_app(app: axum::Router, address: SocketAddr) -> Result<(), Error> {
    let listener = TcpListener::bind(&address).await?;

    let api_address = listener.local_addr()?;

    tracing::info!(
        "API running on address http://{api_address}/api/{API_VERSION}",
        API_VERSION = yattt_backend::API_VERSION
    );
    tracing::info!(
        "API-Docs running on address http://{api_address}/api/{API_VERSION}{DOCS_ROOT_ROUTE}",
        API_VERSION = yattt_backend::API_VERSION,
        DOCS_ROOT_ROUTE = yattt_backend::DOCS_ROOT_ROUTE
    );

    axum::serve(listener, app.into_make_service()).await
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    // load the env variables from .env
    dotenvy::dotenv().ok();

    register_logger();

    let credentials = DatabaseCredentials {
        username: &yattt_backend::DB_USERNAME,
        password: &yattt_backend::DB_PASSWORD,
    };

    #[cfg(not(feature = "test"))]
    let db_backend = SurrealDbBackend::new(
        &yattt_backend::DATABASE_URL,
        credentials,
        yattt_backend::db::db_constants::NAMESPACE,
        yattt_backend::db::db_constants::DATABASE,
    )
    .await
    .expect("Failed to initialize SurrealDB");

    #[cfg(feature = "test")]
    let db = surrealdb::Surreal::new::<Mem>(()).await.unwrap();
    #[cfg(feature = "test")]
    db.use_ns("test_ns").use_db("Testing DB").await.unwrap();
    #[cfg(feature = "test")]
    let db_backend = crate::db::surrealdb::SurrealDbBackend { client: db };

    let app_state = AppState::<yattt_backend::YatttBackend> {
        db: std::sync::Arc::new(db_backend),
    };

    let app = yattt_backend::routes::setup_routes(app_state);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, yattt_backend::APPLICATION_PORT));

    run_app(app, address).await
}
