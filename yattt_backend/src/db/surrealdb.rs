use crate::db::db_constants::*;
use crate::models::user::{User, UserIdExtractor};
use crate::routes::auth::SignInData;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Error as SurrealDbError;
use surrealdb::Surreal;

#[derive(Debug, Clone)]
pub struct SurrealDbBackend<C: surrealdb::Connection = Client> {
    client: Surreal<C>,
}

impl SurrealDbBackend<Client> {
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

impl<C: surrealdb::Connection> super::repositories::UserRepository for SurrealDbBackend<C> {
    type Error = SurrealDbError;
    async fn create_user(&self, user: SignInData) -> Result<Option<User>, Self::Error> {
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
        let query = format!("SELECT * FROM {TABLE_USER} WHERE id = User:{id}");
        let mut result = self.client.query(query).await?.check()?;
        let res: Option<User> = result.take(0)?;

        Ok(res)
    }

    async fn get_by_tag_id(&self, tag_id: &str) -> Result<Option<UserIdExtractor>, Self::Error> {
        let query = format!(
            "SELECT {ENTRY_USER_ID} FROM {TABLE_CARD} WHERE {ENTRY_TAG_ID} = '{}'",
            tag_id
        );
        let mut result = self.client.query(query).await?.check()?;
        let res: Option<UserIdExtractor> = result.take(0)?;
        Ok(res)
    }

    async fn get_by_username(&self, username: &str) -> Result<Option<User>, Self::Error> {
        let query = format!("SELECT * FROM {TABLE_USER} WHERE {ENTRY_USERNAME} = '{username}'");
        let mut result = self.client.query(query).await?.check()?;

        let res: Option<User> = result.take(0)?;

        Ok(res)
    }
}

impl<C: surrealdb::Connection> super::repositories::CardRepository for SurrealDbBackend<C> {
    type Error = SurrealDbError;

    async fn create_card(
        &self,
        card: crate::models::card::Card,
    ) -> Result<Option<crate::models::card::Card>, Self::Error> {
        let query = format!(
            "INSERT INTO {TABLE_CARD} ({ENTRY_USER_ID}, {ENTRY_TAG_ID}, {ENTRY_CARD_NAME}) VALUES ({}, '{}', '{}')",
            card.user_id, card.tag_id, card.card_name
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Option<crate::models::card::Card> = result.take(0)?;

        Ok(res)
    }

    async fn get_cards(
        &self,
        user_id: &str,
    ) -> Result<Vec<crate::models::card::Card>, Self::Error> {
        let query = format!(
            "SELECT * FROM {TABLE_CARD} WHERE {ENTRY_USER_ID} = {TABLE_USER}:{USER_ID};",
            USER_ID = user_id
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Vec<crate::models::card::Card> = result.take(0)?;

        Ok(res)
    }

    async fn update_card(
        &self,
        card_id: &str,
        card: crate::models::card::Card,
        user_id: &str,
    ) -> Result<Option<crate::models::card::Card>, Self::Error> {
        let query = format!(
            "UPDATE {TABLE_CARD} SET {ENTRY_USER_ID} = User:{}, {ENTRY_TAG_ID} = '{}', {ENTRY_CARD_NAME} = '{}' WHERE id = {TABLE_CARD}:{}",
            user_id, card.tag_id, card.card_name, card_id
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Option<crate::models::card::Card> = result.take(0)?;

        Ok(res)
    }

    async fn delete_card(&self, card_id: &str, user_id: &str) -> Result<(), Self::Error> {
        let query = format!("DELETE FROM {TABLE_CARD} WHERE id = {TABLE_CARD}:{} AND {ENTRY_USER_ID} = {TABLE_USER}:{};", card_id, user_id);

        self.client.query(query).await?.check()?;

        Ok(())
    }
}

impl<C: surrealdb::Connection> super::repositories::AttendanceRepository for SurrealDbBackend<C> {
    type Error = SurrealDbError;

    async fn create_attendance(
        &self,
        attendance: crate::models::attendance::Attendance,
    ) -> Result<Option<crate::models::attendance::Attendance>, Self::Error> {
        let query = format!(
            "INSERT INTO {TABLE_ATTENDANCE} ({ENTRY_USER_ID}, {ENTRY_DEVICE_ID}, {ENTRY_CHECK_IN_TIME}, {ENTRY_CHECK_OUT_TIME}, {ENTRY_DURATION}) VALUES ({TABLE_USER}:{}, '{}', d'{}', d'{}', {})",
            attendance.user_id.id, attendance.device_id, attendance.check_in_time, attendance.check_out_time, attendance.duration
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Option<crate::models::attendance::Attendance> = result.take(0)?;

        Ok(res)
    }

    async fn get_attendances(
        &self,
        user_id: &str,
    ) -> Result<Vec<crate::models::attendance::Attendance>, Self::Error> {
        let query = format!(
            "SELECT * FROM {TABLE_ATTENDANCE} WHERE {ENTRY_USER_ID} = {TABLE_USER}:{USER_ID};",
            USER_ID = user_id
        );

        let mut result = self.client.query(query).await?.check()?;

        let res: Vec<crate::models::attendance::Attendance> = result.take(0)?;

        Ok(res)
    }
}

impl From<SurrealDbError> for crate::error::AppError {
    fn from(err: SurrealDbError) -> Self {
        match err {
            SurrealDbError::Api(ref api_err) => match api_err {
                surrealdb::error::Api::Query(ref msg) if msg.contains("not found") => {
                    crate::error::AppError::NotFound
                }
                _ => crate::error::AppError::DatabaseError(err.to_string()),
            },
            _ => crate::error::AppError::DatabaseError(err.to_string()),
        }
    }
}
