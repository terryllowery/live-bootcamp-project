use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
pub async fn should_return_422_if_malformed_request() {

    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [serde_json::json!({
        "password": "password123",
        "require_2fs": true
    })];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input {:?}",
            test_case
        )
    }
}

#[tokio::test]
pub async fn should_return_201_if_valid_input() {
    todo!()
}