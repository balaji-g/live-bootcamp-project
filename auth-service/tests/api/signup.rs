use crate::helpers::TestApp;

#[tokio::test]
async fn signup_test() {
    let app = TestApp::new().await;
    let email = TestApp::get_random_email();
    let test_cases = [
        serde_json::json!({
            "password": "12345678",
            "requires2FA": true
        }),
    ];
    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

