use crate::models::user::User;

#[allow(async_fn_in_trait)]
pub trait UserRepository {
    type Error;
    async fn create(
        &self,
        user: crate::routes::auth::SignInData,
    ) -> Result<Option<User>, Self::Error>;
    async fn get_by_id(&self, id: &str) -> Result<Option<User>, Self::Error>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, Self::Error>;
}

#[allow(async_fn_in_trait)]
pub trait CardRepository {
    type Error;
    async fn create(
        &self,
        card: crate::models::card::Card,
    ) -> Result<Option<crate::models::card::Card>, Self::Error>;
    async fn get_cards(&self) -> Result<Vec<crate::models::card::Card>, Self::Error>;
    async fn update_card(
        &self,
        card_id: &str,
    ) -> Result<Option<crate::models::card::Card>, Self::Error>;
    async fn delete_card(
        &self,
        card_id: &str,
    ) -> Result<Option<crate::models::card::Card>, Self::Error>;
}
