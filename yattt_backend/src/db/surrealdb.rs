use std::sync::LazyLock;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Value;
use surrealdb::Surreal;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn connect(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>(url).await?;

    // Sign in with a username and password
    DB.signin(Root {
        username: &crate::DB_USERNAME, // Replace with your username
        password: &crate::DB_PASSWORD, // Replace with your password
    })
    .await
    .expect("Failed to sign in to SurrealDB");

    // Select the namespace and database
    DB.use_ns("yattt_backend")
        .use_db("yattt_backend")
        .await
        .expect("Failed to select namespace and database");
    Ok(())
}

use crate::models::user::User;
use surrealdb::opt::Resource;
pub async fn create_user(user: User) -> Result<Option<User>, surrealdb::Error> {
    // Insert the user into the database
    let query = format!(
        "INSERT INTO User (username, password) VALUES ('{}', '{}')",
        user.username, user.password
    );

    let mut result = DB.query(query).await?.check()?;

    dbg!(&result);

    let res: Option<User> = result.take(0)?;

    dbg!(&res);

    Ok(res)
}

pub async fn check_user(username: &str) -> Result<Option<User>, surrealdb::Error> {
    // Query to find a matching user
    let query = format!("SELECT * FROM User WHERE username = '{}'", username);

    let mut response = DB.query(&query).await?.check()?;

    if response.num_statements() > 0_usize {
        return response.take::<Option<User>>(0);
    }

    Ok(None) // No matching user
}

pub async fn check_user_by_id(user_id: &str) -> Result<Option<User>, surrealdb::Error> {
    // Query to find a matching user
    let query = format!("SELECT * FROM ONLY User:{};", user_id);

    let mut response = DB.query(&query).await?.check()?;

    if response.num_statements() > 0_usize {
        return response.take::<Option<User>>(0);
    }

    Ok(None) // No matching user
}

