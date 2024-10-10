use crate::TestApp;
#[tokio::test]
pub async fn signup_returns_200() {
    let app = TestApp::new().await;

    let response = app.signup().await;

    assert_eq!(response.status().as_u16(), 200);
    //assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
}
