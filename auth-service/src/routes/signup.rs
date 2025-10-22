use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User},
    services::UserStoreError,
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    // Validate email: must not be empty and must contain '@'
    if email.is_empty() || !email.contains('@') {
        return Err(AuthAPIError::InvalidCredentials);
    }

    // Validate password: must be at least 8 characters
    if password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    // Handle the result from add_user
    match user_store.add_user(user) {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            Ok((StatusCode::CREATED, response))
        }
        Err(UserStoreError::UserAlreadyExists) => Err(AuthAPIError::UserAlreadyExists),
        Err(_) => Err(AuthAPIError::UnexpectedError),
    }
}