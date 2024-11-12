use crate::helpers::TestApp;
#[tokio::test]
pub async fn logout_should_return_200() {
    let app = TestApp::new().await;

    let response = app.logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
pub async fn should_return_400_when_cookie_is_missing() {
    todo!()
}

#[tokio::test]
pub async fn should_return_401_when_cookie_is_invalid() {
    todo!()
}
