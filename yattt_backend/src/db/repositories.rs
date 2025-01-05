use crate::models::user::User;

#[allow(async_fn_in_trait)]
pub trait UserRepository {
    type Error;
    async fn create(&self, user: crate::routes::auth::SignInData) -> Result<Option<User>, Self::Error>;
    async fn get_by_id(&self, id: &str) -> Result<Option<User>, Self::Error>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, Self::Error>;
}

