
pub use surrealdb::engine::remote::ws::Client as SurrealClient;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use surrealdb::Response;
use yattt_backend::{
    db::db_constants,
    models::{
        self,
        user::{self, User},
    },
};

pub const APP_HEALTHCHECK_URL: &str = "http://127.0.0.1:8080/api/v1";

pub const DB_ADDRESS: &str = "127.0.0.1";
pub const DB_PORT: u16 = 8000;
pub const DB_USERNAME: &str = "root";
pub const DB_PASSWORD: &str = "root";
pub const DB_NAMESPACE: &str = db_constants::NAMESPACE;
pub const DB_DATABASE: &str = db_constants::DATABASE;

pub const TOKEN_REGEX: &str = r"^(?:[\w-]*\.){2}[\w-]*$";
pub const REGISTER_URL: &str = "http://127.0.0.1:8080/api/v1/auth/register";
pub const CARD_URL: &str = "http://127.0.0.1:8080/api/v1/cards";

pub async fn database() -> Surreal<SurrealClient> {
    // Connect to the server
    let db = Surreal::new::<Ws>(format!("{DB_ADDRESS}:{DB_PORT}"))
        .await
        .expect("Failed to connect to SurrealDB");

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: DB_USERNAME,
        password: DB_PASSWORD,
    })
    .await
    .expect("Failed to sign in to SurrealDB");

    // Select a specific namespace / database
    db.use_ns(DB_DATABASE)
        .use_db(DB_NAMESPACE)
        .await
        .expect("Failed to select namespace and database");
    db
}

pub use reqwest::Client as ReqwestClient;

pub fn get_http_client() -> ReqwestClient {
    ReqwestClient::new()
}

use tokio::time::{sleep, Duration};

pub async fn wait_for_server(
    client: &ReqwestClient,
    url: &str,
    timeout_secs: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut retries = 0;
    let max_retries = timeout_secs * 10; // Retry every 100ms

    println!("Waiting for server at {} to be online...", url);

    while retries < max_retries {
        if let Ok(response) = client.get(url).send().await {
            if response.status().is_success() {
                println!("Server is online.");
                return Ok(());
            }
        }

        retries += 1;
        sleep(Duration::from_millis(100)).await; // Wait 100ms before retrying
    }

    Err("Server did not become available within the timeout.".into())
}

pub async fn deserialize_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> T {
    let res = response.json::<T>().await;
    assert!(Result::is_ok(&res));
    // the following except should never panic as the result is asserted to be ok
    res.expect("Failed to deserialize response")
}

pub async fn teardown_db(db: surrealdb::Surreal<surrealdb::engine::remote::ws::Client>) {
    let res = db.query("REMOVE DATABASE test;").await;
    assert!(Result::is_ok(&res));
}

pub fn assert_with_regex(property: &str, re: &str) {
    assert!(regex::Regex::new(re).unwrap().is_match(property));
}
