use std::sync::LazyLock;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Error as SurrealDbError;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

// Database related constants
const NAMESPACE: &str = "yattt_backend";
const DATABASE: &str = "yattt_backend";

// Table related constants
const TABLE_USER: &str = "User";
const TABLE_ATTENDANCE: &str = "Attendance";
const TABLE_CARD: &str = "Card";
const TABLE_LECTURE: &str = "Lecture";

// Table entry related constants
const ENTRY_USERNAME: &str = "username";
const ENTRY_PASSWORD: &str = "password";
const ENTRY_USER_ID: &str = "user_id";
const ENTRY_DEVICE_ID: &str = "device_id";
const ENTRY_TAG_ID: &str = "tag_id";
const ENTRY_CARD_NAME: &str = "card_name";
const ENTRY_CHECK_IN_TIME: &str = "check_in_time";
const ENTRY_CHECK_OUT_TIME: &str = "check_out_time";
const ENTRY_DURATION: &str = "duration";
const ENTRY_LV_NAME: &str = "lv_name";
const ENTRY_START_TIME: &str = "start_time";
const ENTRY_END_TIME: &str = "end_time";

#[derive(Debug, Clone)]
pub struct SurrealDbBackend {
    client: Surreal<Client>,
}

impl SurrealDbBackend {
    pub async fn new(
        url: &str,
        credentials: surrealdb::opt::auth::Root<'_>,
        namespace: &str,
        database: &str,
    ) -> Result<Self, SurrealDbError> {
        let client = Surreal::new::<Ws>(url).await?;
        client
            .signin(surrealdb::opt::auth::Root {
                username: credentials.username,
                password: credentials.password,
            })
            .await?;
        client.use_ns(namespace).use_db(database).await?;
        Ok(Self { client })
    }
}


impl UserRepository for SurrealDbBackend {
    type Error = SurrealDbError;
    async fn create(&self, user: SignInData) -> Result<Option<User>, Self::Error> {
        // Insert the user into the database
        let query = format!(
            "INSERT INTO {TABLE_USER} ({ENTRY_USERNAME}, {ENTRY_PASSWORD}) VALUES ('{}', '{}')",
            user.username, user.password
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Option<User> = result.take(0)?;

        Ok(res)
    }
    async fn get_by_id(&self, id: &str) -> Result<Option<User>, Self::Error> {
        let query = format!("SELECT * FROM {TABLE_USER} WHERE {ENTRY_USER_ID} = '{id}'");
        let mut result = self.client.query(query).await?.check()?;

        let res: Option<User> = result.take(0)?;

        Ok(res)
    }
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, Self::Error> {
        let query = format!("SELECT * FROM {TABLE_USER} WHERE {ENTRY_USERNAME} = '{username}'");
        let mut result = self.client.query(query).await?.check()?;

        let res: Option<User> = result.take(0)?;

        Ok(res)
    }
}


impl From<SurrealDbError> for crate::error::AppError {
    fn from(err: SurrealDbError) -> Self {
        match err {
            SurrealDbError::Api(ref api_err) => match api_err {
                surrealdb::error::Api::Query(ref msg) if msg.contains("not found") => crate::error::AppError::NotFound,
                _ => crate::error::AppError::DatabaseError(err.to_string()),
            },
            _ => crate::error::AppError::DatabaseError(err.to_string()),
        }
    }
}




use crate::models::user::User;
use crate::routes::auth::SignInData;

// use crate::models::attendance::{self, Attendance};
// use crate::models::card::Card;

use super::repositories::UserRepository;
