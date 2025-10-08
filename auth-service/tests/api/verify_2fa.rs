use crate::helpers::TestApp;

#[tokio::test]
async fn verify_f2a_test() {
    let app = TestApp::new().await;
    let response = app.get_verify_f2a().await;
    assert_eq!(response.status().as_u16(), 200);
}

