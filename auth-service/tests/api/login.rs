use crate::helpers::{get_random_email, TestApp};
use auth_service::{utils::constants::JWT_COOKIE_NAME, ErrorResponse};

#[tokio::test]
async fn should_return_200_if_valid_credentials_add_2fa_disabled() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password12345",
        "requires2FA": false,
    });

    let respoonse = app.post_signup(&signup_body).await;
    assert_eq!(respoonse.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password12345",
        "requires2FA": false,
    });

    let response = app.login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;
    let random_email = get_random_email();

    // Create user
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password1234",
        "requires2FA": false,
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // Write a test case for each of the invalid inputs
    // 1. Invalid email
    // 2.Invalid password
    // 3. Empty Email
    // 4. Empty Password
    // 5. empty email and password
    let test_cases = vec![
        ("invalid_email", "password123"),
        (random_email.as_str(), "invalid"),
        ("", "password123"),
        (random_email.as_str(), ""),
        ("", ""),
    ];
    for (email, password) in test_cases {
        let login_body = serde_json::json!({
            "email": email,
            "password": password,
        });

        let response = app.login(&login_body).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input {:?}",
            login_body
        );
    }
}

#[tokio::test]
async fn should_return_401_if_invalid_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;
    let random_email = get_random_email();
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password1234",
        "requires2FA": false,
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let test_cases = vec![
        (random_email.as_str(), "wrong-password"),
        ("wrong@email.com", "password123"),
        ("wrong@email.com", "wrong-password"),
    ];

    for (email, password) in test_cases {
        let login_body = serde_json::json!({
            "email": email,
            "password": password,
        });
        let response = app.login(&login_body).await;
        assert_eq!(
            response.status().as_u16(),
            401,
            "Failed for input {:?}",
            login_body
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Incorrect credentials".to_owned()
        );
    }
}
#[tokio::test]
pub async fn should_return_422_when_malformed_credentials() {
    let app = TestApp::new().await;
    let random_email = get_random_email();
    // gen a 201 test case
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password1234",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    // gen a 422 test case
    let test_cases = [
        serde_json::json!({
            "password": "password123",
        }),
        serde_json::json!({
            "email": random_email,
        }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input {:?}",
            test_case
        );
    }
}
