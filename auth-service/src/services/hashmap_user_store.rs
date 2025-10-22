use std::collections::HashMap;

use crate::domain::{User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        // Return `UserStoreError::UserNotFound` if the user can not be found.
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserNotFound` if the user can not be found.
        // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            true,
        );

        // Test adding a user successfully
        let result = store.add_user(user.clone()).await;
        assert_eq!(result, Ok(()));

        // Test adding the same user again should fail
        let result = store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            true,
        );

        // Test getting a user that doesn't exist
        let result = store.get_user("test@example.com").await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));

        // Add user and test getting it
        store.add_user(user.clone()).await.unwrap();
        let result = store.get_user("test@example.com").await;
        assert_eq!(result, Ok(user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "test@example.com".to_string(),
            "password123".to_string(),
            true,
        );

        // Test validating a user that doesn't exist
        let result = store.validate_user("test@example.com", "password123").await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));

        // Add user and test validating with correct password
        store.add_user(user).await.unwrap();
        let result = store.validate_user("test@example.com", "password123").await;
        assert_eq!(result, Ok(()));

        // Test validating with incorrect password
        let result = store.validate_user("test@example.com", "wrongpassword").await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));
    }
}
