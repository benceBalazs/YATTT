use crate::db::db_constants::*;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Error as SurrealDbError;
use surrealdb::Surreal;

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


impl super::repositories::UserRepository for SurrealDbBackend {
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

impl super::repositories::CardRepository for SurrealDbBackend {
    type Error = SurrealDbError;
    
    async fn create(
        &self,
        card: crate::models::card::Card,
    ) -> Result<Option<crate::models::card::Card>, Self::Error> {
        let query = format!(
            "INSERT INTO {TABLE_CARD} ({ENTRY_USER_ID}, {ENTRY_TAG_ID}, {ENTRY_CARD_NAME}) VALUES ('{}', '{}', '{}')",
            card.user, card.tag_id, card.name
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Option<crate::models::card::Card> = result.take(0)?;

        Ok(res)
    }
    
    async fn get_cards(&self) -> Result<Vec<crate::models::card::Card>, Self::Error> {
        let query = format!("SELECT * FROM {TABLE_CARD}");

        let mut result = self.client.query(query).await?.check()?;

        let res: Vec<crate::models::card::Card> = result.take(0)?;

        Ok(res)
    }
    
    async fn update_card(
        &self,
        card_id: &str,
        card: crate::models::card::Card,
    ) -> Result<Option<crate::models::card::Card>, Self::Error> {
        let query = format!(
            "UPDATE {TABLE_CARD} SET {ENTRY_USER_ID} = '{}', {ENTRY_TAG_ID} = '{}', {ENTRY_CARD_NAME} = '{}' WHERE id = '{}'",
            card.user, card.tag_id, card.name, card_id
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Option<crate::models::card::Card> = result.take(0)?;

        Ok(res)
    }
    
    async fn delete_card(
        &self,
        card_id: &str,
    ) -> Result<Option<crate::models::card::Card>, Self::Error> {
        let query = format!("DELETE FROM {TABLE_CARD} WHERE id = '{}'", card_id);

        let mut result = self.client.query(query).await?.check()?;

        let res: Option<crate::models::card::Card> = result.take(0)?;

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

