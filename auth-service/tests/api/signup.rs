use auth_service::routes::signup::SignupResponse;
use crate::helpers::TestApp;

#[tokio::test]
async fn signup_test() {
    let app = TestApp::new().await;
    let _email = TestApp::get_random_email();
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

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let email = TestApp::get_random_email();
    
    let request_body = serde_json::json!({
        "email": email,
        "password": "password123",
        "requires2FA": true
    });
    
    let response = app.post_signup(&request_body).await;
    
    assert_eq!(
        response.status().as_u16(),
        201,
        "Failed to create user with valid input"
    );

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}
