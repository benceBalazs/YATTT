// lib to share types between the tests
pub mod db;
pub mod models;
pub mod routes;
pub mod jwt;
pub mod encryption;
pub mod error;

#[macro_use]
mod macros;

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

pub const YATTT_TAG: &str = "YATTT";
pub const API_VERSION: &str = "v1";
pub const APPLICATION_PORT: u16 = 8080;
pub const DOCS_ROOT_ROUTE: &str = "/docs";

pub type YatttAppState = AppState<YatttBackend>;
#[cfg(not(feature = "test"))]
pub type YatttEncrypter = crate::encryption::BcryptPasswordEncrypter;
#[cfg(feature = "test")]
pub type YatttEncrypter = crate::encryption::TestPasswordEncrypter;
#[cfg(not(feature = "test"))]
pub type YatttEncoder = crate::jwt::JWTEncoder;
#[cfg(feature = "test")]
pub type YatttEncoder = crate::jwt::TestJWTEncoder;

pub trait Backend {
    type Db: db::repositories::UserRepository + db::repositories::CardRepository + db::repositories::AttendanceRepository;
}

#[derive(Clone)]
pub struct YatttBackend;

impl Backend for YatttBackend {
    type Db = db::surrealdb::SurrealDbBackend;
}

#[derive(Clone)]
pub struct AppState<B: Backend> {
    pub db: std::sync::Arc<B::Db>,
}