use std::collections::HashMap;
use crate::domain::User;


#[derive(Debug)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let user_email = user.email.clone();
        if self.users.contains_key(&user_email) {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            self.users.insert(user_email, user);
            return Ok(());
        }
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user),
            None => Err(UserStoreError::UserNotFound),            
        }
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
         match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }, None => Err(UserStoreError::UserNotFound)
         }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    //use std::collections::HashMap;

    #[tokio::test]
    async fn test_add_user() {
        let user = User::new(
            "john@smith.com".to_string(),
            "password123".to_string(),
            true,
        );
        let mut user_store = HashmapUserStore::default();
        _ = user_store.add_user(user);

        let user_email = "john@smith.com".to_string();

        let retrived_user = user_store.get_user(user_email.as_str());
        
        assert_eq!(retrived_user.unwrap().email, user_email);
  
    }

    #[tokio::test]
    async fn test_get_user() {
        let user = User::new(
            "john@smith.com".to_string(),
            "password123".to_string(),
            true,
        );
        let mut user_store = HashmapUserStore::default();
        _ = user_store.add_user(user);
        let user_email = "john@smith.com".to_string();
        

        let retrived_user = user_store.get_user(user_email.as_str());

        assert_eq!(retrived_user.unwrap().email, user_email);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let user = User::new(
            "john@smith.com".to_string(),
            "password123".to_string(),
            true,
        );
        let mut user_store = HashmapUserStore::default();
        _ = user_store.add_user(user);

        let user_email = "john@smith.com".to_string();

        let retrived_user = user_store.validate_user(user_email.as_str(), "password123");

        assert_eq!(retrived_user.unwrap(), ());
}
}