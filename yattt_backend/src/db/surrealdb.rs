use crate::db::db_constants::*;
use crate::models::user::{User, UserIdExtractor};
use crate::routes::auth::SignInData;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Error as SurrealDbError;
use surrealdb::Surreal;

#[derive(Debug, Clone)]
pub struct SurrealDbBackend<C: surrealdb::Connection = Client> {
    pub client: Surreal<C>,
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
            "INSERT INTO {TABLE_ATTENDANCE} ({ENTRY_USER_ID}, {ENTRY_TAG_ID} ,{ENTRY_DEVICE_ID}, {ENTRY_CHECK_IN_TIME}, {ENTRY_CHECK_OUT_TIME}, {ENTRY_DURATION}) VALUES ({TABLE_USER}:{}, '{}', '{}', d'{}', d'{}', {})",
            attendance.user_id.id, attendance.tag_id ,attendance.device_id, attendance.check_in_time, attendance.check_out_time, attendance.duration
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

        print!("{}", query);

        let mut result = self.client.query(query).await?.check()?;

        print!("{:?}", result);

        let res: Vec<crate::models::attendance::Attendance> = result.take(0)?;

        Ok(res)
    }

    async fn get_lectures_by_device_id(
        &self,
        device_id: &str,
    ) -> Result<Vec<crate::models::lecture::Lecture>, Self::Error> {
        let query = format!(
            "SELECT * FROM {TABLE_LECTURE} WHERE {ENTRY_DEVICE_ID} = '{}';",
            device_id
        );

        println!("{:?}", query);

        let mut result = self.client.query(query).await?.check()?;

        let res: Vec<crate::models::lecture::Lecture> = result.take(0)?;

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

#[cfg(test)]
mod tests {
    use crate::db::repositories::{CardRepository, UserRepository};
    use crate::db::surrealdb::SurrealDbBackend;
    use crate::models::card::Card;
    use crate::models::user::User;
    use crate::routes::auth::SignInData;
    use surrealdb::engine::local::{Db as SurrealDb, Mem};
    use surrealdb::sql::{Id, Thing};
    type UsedDB = SurrealDbBackend<SurrealDb>;

    static NON_EXISTENT: &str = "non_existent";
    static TEST_USER_USERNAME: &str = "test_user";
    static TEST_USER_PASSWORDHASH: &str = "test_passwordhash";
    static TEST_TAG_ID: &str = "test_tag_id";
    static TEST_CARD_NAME: &str = "test_card_name";
    static TABLE_USER: &str = "User";
    static UPDATED_ENTRY: &str = "updated";
    static TEST_DEVICE_ID: &str = "test_device_id";

    static EXPECT_NO_DB_ERROR: &str = "No database error should occur";
    static EXPECT_USER_CREATED: &str = "User should be created";
    static EXPECT_USER_HAS_ID: &str = "User should have an id";

    static EXPECT_CARD_CREATED: &str = "Card should be created";
    static EXPECT_CARD_HAS_ID: &str = "Card should have an id";

    async fn with_db(name: &str) -> UsedDB {
        let db = surrealdb::Surreal::new::<Mem>(()).await.unwrap();

        db.use_ns("test_ns").use_db(name).await.unwrap();

        SurrealDbBackend { client: db }
    }

    async fn create_test_user(db: &UsedDB) -> User {
        db.create_user(SignInData {
            username: TEST_USER_USERNAME.to_owned(),
            password: TEST_USER_PASSWORDHASH.to_string(),
        })
        .await
        .expect(EXPECT_NO_DB_ERROR)
        .expect(EXPECT_USER_CREATED)
    }

    fn get_user_id(user: User) -> Id {
        user.id.expect(EXPECT_USER_HAS_ID).id
    }

    fn get_card_id(card: Card) -> Id {
        card.id.expect(EXPECT_CARD_HAS_ID).id
    }

    async fn create_test_card(
        db: &UsedDB,
        user_id: Option<Thing>,
        tag_id: Option<&str>,
        card_name: Option<&str>,
    ) -> Card {
        db.create_card(crate::models::card::Card {
            id: None,
            user_id: user_id.unwrap_or(Thing::from((TABLE_USER, TEST_USER_USERNAME))),
            tag_id: tag_id.unwrap_or(TEST_TAG_ID).to_owned(),
            card_name: card_name.unwrap_or(TEST_CARD_NAME).to_owned(),
        })
        .await
        .expect(EXPECT_NO_DB_ERROR)
        .expect(EXPECT_CARD_CREATED)
    }

    mod user_repository {
        use pretty_assertions::assert_eq;

        use crate::{
            db::{
                repositories::{CardRepository, UserRepository},
                surrealdb::tests::{
                    create_test_card, create_test_user, get_user_id, with_db, EXPECT_NO_DB_ERROR,
                    EXPECT_USER_CREATED, EXPECT_USER_HAS_ID, NON_EXISTENT, TEST_CARD_NAME,
                    TEST_TAG_ID, TEST_USER_PASSWORDHASH, TEST_USER_USERNAME,
                },
            },
            models::card::Card,
            routes::auth::SignInData,
        };

        #[tokio::test]
        async fn create_user_returns_a_user() {
            let db = with_db("create_user_returns_a_user").await;

            let created_user = db
                .create_user(SignInData {
                    username: TEST_USER_USERNAME.to_string(),
                    password: TEST_USER_PASSWORDHASH.to_string(),
                })
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert!(created_user.is_some());
        }

        #[tokio::test]
        async fn create_user_returns_username_of_the_created_user() {
            let db = with_db("create_user_returns_username_of_the_created_user").await;

            let created_user = create_test_user(&db).await;

            assert!(created_user.username == TEST_USER_USERNAME);
        }

        #[tokio::test]
        async fn create_user_returns_password_of_the_created_user() {
            let db = with_db("create_user_returns_password_of_the_created_user").await;

            let created_user = create_test_user(&db).await;

            assert!(created_user.password == TEST_USER_PASSWORDHASH);
        }

        #[tokio::test]
        async fn get_by_id_no_found_user_results_in_none() {
            let db = with_db("get_by_id_no_found_user_results_in_none").await;

            let found_user = db.get_by_id(NON_EXISTENT).await.expect(EXPECT_NO_DB_ERROR);

            assert!(found_user.is_none());
        }

        #[tokio::test]
        async fn get_by_id_returns_a_user() {
            let db = with_db("get_by_id_returns_a_user").await;

            let created_user = create_test_user(&db).await;

            let user_id = get_user_id(created_user);

            let found_user = db.get_by_id(&user_id.to_string()).await.unwrap();

            assert!(found_user.is_some());
        }

        #[tokio::test]
        async fn get_by_id_returns_user_with_specified_id() {
            let db = with_db("get_by_id_returns_user_with_specified_id").await;

            let created_user = create_test_user(&db).await;

            let user_id = get_user_id(created_user);

            let found_user = db
                .get_by_id(user_id.to_string().as_str())
                .await
                .expect(EXPECT_NO_DB_ERROR)
                .expect(EXPECT_USER_CREATED);

            let found_user_id = get_user_id(found_user);

            assert_eq!(found_user_id, user_id);
        }

        #[tokio::test]
        async fn get_by_username_no_found_user_results_in_none() {
            let db = with_db("get_by_username_no_found_user_results_in_none").await;

            let found_user = db
                .get_by_username(NON_EXISTENT)
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert!(found_user.is_none());
        }

        #[tokio::test]
        async fn get_by_username_returns_a_user() {
            let db = with_db("get_by_username_returns_a_user").await;

            let created_user = create_test_user(&db).await;

            let found_user = db
                .get_by_username(&created_user.username)
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert!(found_user.is_some());
        }

        #[tokio::test]
        async fn get_by_username_returns_user_with_specified_id() {
            let db = with_db("get_by_username_returns_user_with_specified_id").await;

            let created_user = create_test_user(&db).await;

            let user_id = get_user_id(created_user.clone());

            let found_user = db
                .get_by_username(&created_user.username)
                .await
                .expect(EXPECT_NO_DB_ERROR)
                .expect(EXPECT_USER_CREATED);

            let found_user_id = get_user_id(found_user);

            assert_eq!(found_user_id, user_id);
        }

        #[tokio::test]
        async fn get_by_tag_id_no_found_user_results_in_none() {
            let db = with_db("get_by_tag_id_no_found_user_results_in_none").await;

            let found_user = db
                .get_by_tag_id(NON_EXISTENT)
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert!(found_user.is_none());
        }

        #[tokio::test]
        async fn get_by_tag_id_returns_a_user_id() {
            let db = with_db("get_by_tag_id_returns_a_user_id").await;

            let created_user = create_test_user(&db).await;

            let tag_id = TEST_TAG_ID.to_string();

            db.create_card(Card {
                id: None,
                user_id: created_user.id.expect(EXPECT_USER_HAS_ID),
                tag_id: TEST_TAG_ID.to_string(),
                card_name: TEST_CARD_NAME.to_string(),
            })
            .await
            .expect(EXPECT_NO_DB_ERROR);

            let a = db.get_by_tag_id(&tag_id).await.unwrap();

            assert!(a.is_some());
        }

        #[tokio::test]
        async fn get_by_tag_id_returns_specific_user_id() {
            let db = with_db("get_by_tag_id_returns_specific_user_id").await;

            let created_user = create_test_user(&db).await;

            let tag_id = TEST_TAG_ID.to_string();

            let user_id = created_user.id.expect(EXPECT_USER_HAS_ID);

            create_test_card(&db, Some(user_id.clone()), Some(&tag_id), None).await;

            let found_user = db
                .get_by_tag_id(&tag_id)
                .await
                .expect(EXPECT_NO_DB_ERROR)
                .expect(EXPECT_USER_CREATED);

            let found_user_id = found_user.user_id.expect(EXPECT_USER_HAS_ID);

            assert_eq!(found_user_id, user_id);
        }
    }
    mod card_repository {
        use pretty_assertions::assert_eq;

        use crate::{
            db::{
                repositories::CardRepository,
                surrealdb::tests::{
                    create_test_card, create_test_user, get_card_id, with_db, EXPECT_CARD_CREATED,
                    EXPECT_NO_DB_ERROR, EXPECT_USER_CREATED, EXPECT_USER_HAS_ID, NON_EXISTENT,
                    TABLE_USER, TEST_CARD_NAME, TEST_TAG_ID, TEST_USER_USERNAME, UPDATED_ENTRY,
                },
            },
            models::card::Card,
        };

        #[tokio::test]
        async fn create_card_returns_a_card() {
            let db = with_db("create_card_returns_a_card").await;

            let created_user = create_test_user(&db).await;

            let user_id = created_user.id.expect(EXPECT_USER_HAS_ID);

            let created_card = db
                .create_card(Card {
                    id: None,
                    user_id,
                    tag_id: TEST_TAG_ID.to_string(),
                    card_name: TEST_CARD_NAME.to_string(),
                })
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert!(created_card.is_some());
        }

        #[tokio::test]
        async fn create_card_returns_a_card_with_specified_user() {
            let db = with_db("create_card_returns_a_card_with_specified_user").await;

            let res = create_test_user(&db).await;

            let user_id = res.id.expect(EXPECT_USER_HAS_ID);

            let created_card = db
                .create_card(Card {
                    id: None,
                    user_id: user_id.clone(),
                    tag_id: TEST_TAG_ID.to_string(),
                    card_name: TEST_CARD_NAME.to_string(),
                })
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert_eq!(created_card.expect(EXPECT_CARD_CREATED).user_id, user_id);
        }

        #[tokio::test]
        async fn retrieve_cards_returns_no_cards_for_non_existent_user() {
            let db = with_db("retrieve_cards_returns_no_cards_for_non_existent_user").await;

            let cards = db.get_cards(NON_EXISTENT).await.expect(EXPECT_NO_DB_ERROR);
            assert_eq!(cards.len(), 0);
        }

        #[tokio::test]
        async fn retrieve_cards_returns_a_card() {
            let db = with_db("retrieve_cards_returns_a_card").await;

            let user = create_test_user(&db).await;

            let user_id = user.id.expect(EXPECT_USER_HAS_ID);

            let card_1 = create_test_card(
                &db,
                Some(user_id.clone()),
                Some("test_tag1"),
                Some("test_card1"),
            )
            .await;

            let cards = db
                .get_cards(&user_id.id.to_string())
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert_eq!(cards.len(), 1);
            assert_eq!(cards[0].card_name, card_1.card_name);
        }

        #[tokio::test]
        async fn retrieve_cards_returns_both_cards() {
            let db = with_db("retrieve_cards_returns_both_cards").await;

            let user = create_test_user(&db).await;

            let user_id = user.id.expect(EXPECT_USER_HAS_ID);

            create_test_card(
                &db,
                Some(user_id.clone()),
                Some("test_tag1"),
                Some("test_card1"),
            )
            .await;
            create_test_card(
                &db,
                Some(user_id.clone()),
                Some("test_tag2"),
                Some("test_card2"),
            )
            .await;

            let cards = db
                .get_cards(&user_id.id.to_string())
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert_eq!(cards.len(), 2);
        }

        #[tokio::test]
        async fn modify_card_returns_none_for_non_existent_card_id() {
            let db = with_db("modify_card_returns_none_for_non_existent_card_id").await;

            let user_id = surrealdb::sql::Thing::from((TABLE_USER, TEST_USER_USERNAME));

            let updated_card = db
                .update_card(
                    NON_EXISTENT,
                    Card {
                        id: None,
                        user_id,
                        tag_id: TEST_TAG_ID.to_string(),
                        card_name: TEST_CARD_NAME.to_string(),
                    },
                    NON_EXISTENT,
                )
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert!(updated_card.is_none());
        }

        #[tokio::test]
        async fn modify_card_returns_some() {
            let db = with_db("modify_card_returns_some").await;

            let user_id = surrealdb::sql::Thing::from((TABLE_USER, TEST_USER_USERNAME));

            let created_card = create_test_card(&db, Some(user_id.clone()), None, None).await;

            let card_id = get_card_id(created_card);

            let updated_card = db
                .update_card(
                    &card_id.to_string(),
                    Card {
                        id: None,
                        user_id,
                        tag_id: UPDATED_ENTRY.to_string(),
                        card_name: UPDATED_ENTRY.to_string(),
                    },
                    TEST_USER_USERNAME,
                )
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert!(updated_card.is_some());
        }

        #[tokio::test]
        async fn modify_card_returns_updated_cardname() {
            let db = with_db("modify_card_returns_updated_cardname").await;

            let user_id = surrealdb::sql::Thing::from((TABLE_USER, TEST_USER_USERNAME));

            let created_card = create_test_card(&db, Some(user_id.clone()), None, None).await;

            let card_id = get_card_id(created_card);

            let updated_card = db
                .update_card(
                    &card_id.to_string(),
                    Card {
                        id: None,
                        user_id,
                        tag_id: UPDATED_ENTRY.to_string(),
                        card_name: UPDATED_ENTRY.to_string(),
                    },
                    TEST_USER_USERNAME,
                )
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert_eq!(
                updated_card.expect(EXPECT_USER_CREATED).card_name,
                UPDATED_ENTRY
            );
        }

        #[tokio::test]
        async fn delete_card_returns_unit_for_non_existent_card_id() {
            let db = with_db("delete_card_returns_unit_for_non_existent_card_id").await;

            let response = db.delete_card(NON_EXISTENT, NON_EXISTENT).await;

            assert!(response.is_ok());
        }

        #[tokio::test]
        async fn delete_card_returns_unit_when_card_was_deleted() {
            let db = with_db("delete_card_returns_unit_when_card_was_deleted").await;

            let user_id = surrealdb::sql::Thing::from((TABLE_USER, TEST_USER_USERNAME));

            let created_card = create_test_card(&db, Some(user_id.clone()), None, None).await;

            let card_id = get_card_id(created_card);

            let response = db
                .delete_card(&card_id.to_string(), TEST_USER_USERNAME)
                .await;

            assert!(response.is_ok());
        }
    }
    mod attendance_repository {
        use crate::{
            db::{
                repositories::AttendanceRepository,
                surrealdb::tests::{
                    create_test_card, create_test_user, get_user_id, with_db, EXPECT_NO_DB_ERROR,
                    EXPECT_USER_HAS_ID, NON_EXISTENT, TEST_DEVICE_ID, TEST_TAG_ID,
                },
            },
            models::attendance::Attendance,
        };

        #[tokio::test]
        async fn retrieve_attendances_results_empty_vec() {
            let db = with_db("retrieve_attendances_results_empty_vec").await;

            let user = create_test_user(&db).await;

            let user_id = get_user_id(user);

            let attendances = db
                .get_attendances(&user_id.to_string())
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert_eq!(attendances.len(), 0);
            assert_eq!(attendances, Vec::new());
        }

        #[tokio::test]
        async fn retrieve_attendances_results_vec_of_attendances() {
            let db = with_db("retrieve_attendances_results_vec_of_attendances").await;

            let user = create_test_user(&db).await;
            let user_id = user.id.expect(EXPECT_USER_HAS_ID);

            create_test_card(&db, Some(user_id.clone()), None, None).await;

            let attendance = db
                .create_attendance(Attendance {
                    user_id: user_id.clone(),
                    tag_id: TEST_TAG_ID.to_string(),
                    device_id: TEST_DEVICE_ID.to_string(),
                    check_in_time: "2024-12-11T15:30:00Z".to_string(),
                    check_out_time: "2024-12-11T16:00:00Z".to_string(),
                    duration: 1.0,
                    id: None,
                })
                .await
                .expect(EXPECT_NO_DB_ERROR)
                .expect(EXPECT_USER_HAS_ID);

            let attendances = db
                .get_attendances(&user_id.id.to_string())
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert_eq!(attendances.len(), 1);
            assert_eq!(attendances[0], attendance);
        }

        #[tokio::test]
        async fn retrieve_attendances_results_empty_vec_for_non_existent_user_id() {
            let db =
                with_db("retrieve_attendances_results_empty_vec_for_non_existent_user_id").await;

            let attendances = db
                .get_attendances(NON_EXISTENT)
                .await
                .expect(EXPECT_NO_DB_ERROR);

            assert_eq!(attendances.len(), 0);
            assert_eq!(attendances, Vec::new());
        }
    }
}
