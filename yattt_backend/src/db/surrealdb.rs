use std::sync::LazyLock;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Value;
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

pub async fn connect(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>(url).await?;

    // Sign in with a username and password
    DB.signin(Root {
        username: &crate::DB_USERNAME,
        password: &crate::DB_PASSWORD,
    })
    .await
    .expect("Failed to sign in to SurrealDB");

    // Select the namespace and database
    DB.use_ns(NAMESPACE)
        .use_db(DATABASE)
        .await
        .expect("Failed to select namespace and database");
    Ok(())
}

use crate::models::user::User;
use crate::routes::auth::SignInData;
use surrealdb::opt::Resource;

pub async fn create_user(user: SignInData) -> Result<Option<User>, surrealdb::Error> {
    // Insert the user into the database
    let query = format!(
        "INSERT INTO {TABLE_USER} ({ENTRY_USERNAME}, {ENTRY_PASSWORD}) VALUES ('{}', '{}')",
        user.username, user.password
    );

    let mut result = DB.query(query).await?.check()?;

    let res: Option<User> = result.take(0)?;

    Ok(res)
}

pub async fn check_user(username: &str) -> Result<Option<User>, SurrealDbError> {
    // Query to find a matching user
    let query = format!(
        "SELECT * FROM {TABLE_USER} WHERE {ENTRY_USERNAME} = '{}'",
        username
    );

    let mut response = DB.query(&query).await?.check()?;

    if response.num_statements() > 0_usize {
        return response.take::<Option<User>>(0);
    }

    Ok(None) // No matching user
}

pub async fn check_user_by_id(user_id: &str) -> Result<Option<User>, SurrealDbError> {
    // Query to find a matching user
    let query = format!("SELECT * FROM ONLY {TABLE_USER}:{};", user_id);

    let mut response = DB.query(&query).await?.check()?;

    if response.num_statements() > 0_usize {
        return response.take::<Option<User>>(0);
    }

    Ok(None) // No matching user
}

use crate::models::card::Card;

use crate::models::attendance::{self, Attendance};
