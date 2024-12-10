use std::sync::LazyLock;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
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
pub async fn create_user(user: User) -> Option<User> {
    let query_result = DB.create::<Option<User>>("User").content(user).await;

    match query_result {
        Ok(Some(record)) => {
            // Return the record if the result is Ok(Some(record))
            tracing::info!("User created: {:?}", record);

            Some(record)
        }
        Ok(None) => {
            // Handle the case where the result is Ok(None), if needed
            None
        }
        Err(e) => {
            tracing::error!("Error creating user: {}", e);
            None
        }
    }
}
