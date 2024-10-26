use crate::helpers::TestApp;
#[tokio::test]
pub async fn login_should_return_200() {
    let app = TestApp::new().await;

    let response = app.login().await;
    
    assert_eq!(response.status().as_u16(), 200);
}
