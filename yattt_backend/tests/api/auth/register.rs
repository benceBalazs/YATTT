use std::error::Error;

use pretty_assertions::assert_eq;

use yattt_backend::models;

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

    Ok(())
}

mod performance_tests {
    use ::futures::future::join_all;
    use std::time::Duration;
    use tokio::runtime::Runtime;
    use yattt_backend::models;

    use crate::common;

    #[test]
    fn test_performance_register() {
        let runtime = Runtime::new().expect("Failed to create runtime");
        runtime.block_on(async {
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
                .await;

            assert!(response.is_ok());

            let start = std::time::Instant::now();

            let mut handles = Vec::new();

            for _ in 0..100 {
                let client_clone = client.clone();
                let register_data_clone = register_data.clone();

                handles.push(tokio::spawn(async move {
                    let response = client_clone
                        .post(common::REGISTER_URL)
                        .json(&register_data_clone)
                        .send()
                        .await;

                    assert!(response.is_ok());
                }));
            }

            join_all(handles).await;

            let duration = start.elapsed();

            println!("Performance test took: {:?}", duration);

            assert!(duration < Duration::from_secs(5));
        });
    }

    #[test]
    fn test_performance_create_card() {
        let runtime = Runtime::new().expect("Failed to create runtime");
        runtime.block_on(async {
            let client = common::get_http_client();

            // Assuming you have a function to get a valid access token
            let register_data = yattt_backend::models::user::User {
                id: None,
                username: "testuser".to_string(),
                password: "password123".to_string(),
            };

            let response = client
                .post(common::REGISTER_URL)
                .json(&register_data)
                .send()
                .await;

            let access_token = match response {
                Ok(response) => {
                    let d =
                        common::deserialize_response::<models::auth::TokenResponse>(response).await;
                    d.access_token
                }
                Err(_) => panic!("Failed to get access token"),
            };

            let card_data = yattt_backend::models::card::CardRequest {
                tag_id: "test_tag".to_string(),
                card_name: "test_card".to_string(),
            };

            let start = std::time::Instant::now();

            let mut handles = Vec::new();

            for _ in 0..100 {
                let client_clone = client.clone();
                let card_data_clone = card_data.clone();
                let access_token_clone = access_token.clone();

                handles.push(tokio::spawn(async move {
                    let response = client_clone
                        .post(common::CARD_URL)
                        .header("Authorization", format!("Bearer {}", access_token_clone))
                        .json(&card_data_clone)
                        .send()
                        .await;

                    assert!(response.is_ok());
                }));
            }

            join_all(handles).await;

            let duration = start.elapsed();

            println!("Performance test took: {:?}", duration);

            assert!(duration < Duration::from_secs(5));
        });
    }
}
