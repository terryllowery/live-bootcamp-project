use crate::domain::{Email, Password, User, UserStore, UserStoreError};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        return Ok(());
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
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
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: Email::parse("john@smith.com".to_owned()).unwrap(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: false,
        };

        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let user = User {
            email: Email::parse("john@smith.com".to_owned()).unwrap(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: true,
        };
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("john@smith.com".to_owned()).unwrap();

        // Test get on user
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.get_user(&email).await;
        assert_eq!(result.unwrap(), user);

        // Test get on non-existent user
        let result = user_store
            .get_user(&Email::parse("notauser@notdomain.com".to_owned()).unwrap())
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let user = User {
            email: Email::parse("john@smith.com".to_owned()).unwrap(),
            password: Password::parse("password123".to_owned()).unwrap(),
            requires_2fa: true,
        };

        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("john@smith.com".to_owned()).unwrap();
        let password = Password::parse("password123".to_owned()).unwrap();

        // Test validate on user
        user_store.users.insert(user.email.clone(), user.clone());
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // Test validate on bad password
        let wrong_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validate on non-existent user
        let result = user_store
            .validate_user(
                &Email::parse("dont@exists.com".to_string()).unwrap(),
                &password,
            )
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
