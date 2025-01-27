use crate::models::user::{User, UserIdExtractor};

#[allow(async_fn_in_trait)]
pub trait UserRepository {
    type Error;
    async fn create_user(
        &self,
        user: crate::routes::auth::SignInData,
    ) -> Result<Option<User>, Self::Error>;
    async fn get_by_id(&self, id: &str) -> Result<Option<User>, Self::Error>;
    async fn get_by_tag_id(&self, tag_id: &str) -> Result<Option<UserIdExtractor>, Self::Error>;
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, Self::Error>;
}

#[allow(async_fn_in_trait)]
pub trait CardRepository {
    type Error;
    async fn create_card(
        &self,
        card: crate::models::card::Card,
    ) -> Result<Option<crate::models::card::Card>, Self::Error>;
    async fn get_cards(&self, user_id: &str) -> Result<Vec<crate::models::card::Card>, Self::Error>;
    async fn update_card(
        &self,
        card_id: &str,
        card: crate::models::card::Card,
        user_id: &str
    ) -> Result<Option<crate::models::card::Card>, Self::Error>;
    async fn delete_card(
        &self,
        card_id: &str,
        user_id: &str
    ) -> Result<(), Self::Error>;
}

#[allow(async_fn_in_trait)]
pub trait AttendanceRepository {
    type Error;
    async fn create_attendance(
        &self,
        attendance: crate::models::attendance::Attendance,
    ) -> Result<Option<crate::models::attendance::Attendance>, Self::Error>;
    async fn get_attendances(
        &self,
        user_id: &str
    ) -> Result<Vec<crate::models::attendance::Attendance>, Self::Error>;
    async fn get_lectures_by_device_id(
        &self,
        device_id: &str
    ) -> Result<Vec<crate::models::lecture::Lecture>, Self::Error>;
}
