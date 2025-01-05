use std::error::Error;

use pretty_assertions::assert_eq;

use yattt_backend::{db::db_constants, models};

use crate::common;

#[tokio::test]
async fn register_fails_with_no_data() -> Result<(), Box<dyn Error>> {
    let client = common::get_http_client();

    let response = client.post(common::REGISTER_URL).send().await?;

    assert_eq!(response.status(), 415);

    Ok(())
}

#[tokio::test]
async fn register_fails_with_empty_name() -> Result<(), Box<dyn Error>> {
    let client = common::get_http_client();

    let register_data = yattt_backend::models::user::User {
        id: None,
        username: "".to_string(),
        password: "".to_string(),
    };

    let response = client
        .post(common::REGISTER_URL)
        .json(&register_data)
        .send()
        .await?;

    assert_eq!(response.status(), 400);

    Ok(())
}

#[tokio::test]
async fn register_accepts_valid_data() -> Result<(), Box<dyn Error>> {
    let client = common::get_http_client();

    let register_data = yattt_backend::models::user::User {
        id: None,
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };

    let response = client
        .post(common::REGISTER_URL)
        .json(&register_data)
        .send()
        .await?;

    assert_eq!(response.status(), 201);

    Ok(())
}

#[tokio::test]
async fn register_valid_response() -> Result<(), Box<dyn Error>> {
    let client = common::get_http_client();

    let register_data = yattt_backend::models::user::User {
        id: None,
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };

    let response = client
        .post(common::REGISTER_URL)
        .json(&register_data)
        .send()
        .await?;

    let d = common::deserialize_response::<models::auth::TokenResponse>(response).await;

    assert_eq!(d.token_type, "Bearer");
    common::assert_with_regex(&d.access_token, common::TOKEN_REGEX);

    Ok(())
}

// #[tokio::test]
// #[serial_test::serial]
// async fn test_auth_register_route() -> Result<(), Box<dyn Error>> {
//     let client = common::ReqwestClient::new();
//     common::wait_for_server(&client, common::APP_HEALTHCHECK_URL, 1).await?;

//     let db = common::database().await;

//     let register_data = yattt_backend::models::user::User {
//         id: None,
//         username: "testuser".to_string(),
//         password: "password123".to_string(),
//     };

//     // SUT
//     let response = client
//         .post(common::REGISTER_URL)
//         .json(&register_data)
//         .send()
//         .await?;

//     assert_eq!(response.status(), 201);

//     let d = common::deserialize_response::<models::auth::TokenResponse>(response).await;

//     assert_eq!(d.token_type, "Bearer");
//     common::assert_with_regex(&d.access_token, common::TOKEN_REGEX);

//     let mut res = db
//         .query(format!(
//             "SELECT * FROM {user};",
//             user = db_constants::TABLE_USER
//         ))
//         .await?
//         .check()?;

//     assert_eq!(res.num_statements(), 1);

//     assert_eq!(
//         res.take::<Option<String>>(db_constants::ENTRY_USERNAME)?,
//         Some(register_data.username.to_string())
//     );
//     assert_eq!(
//         res.take::<Option<String>>(db_constants::ENTRY_PASSWORD)?,
//         Some(register_data.password.to_string())
//     );

//     let id = res.take::<Option<String>>("id")?.expect("ID not found");

//     db.query(format!(
//         "REMOVE {user}:{id};",
//         user = db_constants::TABLE_USER,
//         id = id
//     ))
//     .await?
//     .check()?;
//     // common::teardown_db(db).await;

//     Ok(())
// }

//what u test, conditions, result
// function_testcase_expected_result
//
// #[tokio::test]
// #[serial_test::serial]
// async fn register_user_created_in_database() -> Result<(), Box<dyn Error>> {
//     let client = common::ReqwestClient::new();
//     common::wait_for_server(&client, common::APP_HEALTHCHECK_URL, 1).await?;
//     let db = common::database().await;

//     let register_data = yattt_backend::models::user::User {
//         id: None,
//         username: "testuser".to_string(),
//         password: "password123".to_string(),
//     };

//     // SUT
//     let response = client
//         .post("http://127.0.0.1:8080/api/v1/auth/login")
//         .json(&register_data)
//         .send()
//         .await?;

//     assert_eq!(response.status(), 200);

//     let d = common::deserialize_response::<models::auth::TokenResponse>(response).await;

//     assert_eq!(d.token_type, "Bearer");
//     common::assert_with_regex(&d.access_token, common::TOKEN_REGEX);

//     // let id= res.take::<Option<String>>("id")?.expect("ID not found");
//     // db.query(format!("REMOVE {user}:{id};", user = db_constants::TABLE_USER, id = id)).await?.check()?;
//     Ok(())
// }
