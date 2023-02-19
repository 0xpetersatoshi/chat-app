use crate::schema::users;
use argon2::{self, Config};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Define a struct to hold the updated values
#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UserChanges {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: Vec<u8>,
}

impl<'a> NewUser<'a> {
    pub fn new(username: &'a str, email: &'a str, password: &'a str) -> Self {
        dotenv().ok();
        let salt = env::var("PASSWORD_HASH_SALT").expect("PASSWORD_HASH_SALT must be set");
        let config = Config::default();
        let password_hash =
            argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
        let password_hash = password_hash.as_bytes().to_vec();

        NewUser {
            username,
            email,
            password_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        // Set up environment variable
        env::set_var("PASSWORD_HASH_SALT", "test-salt-value");

        // Create new user
        let username = "user123";
        let email = "me@email.com";
        let password = "strong-password-1234";
        let user = NewUser::new(username, email, password);

        // Verify fields are set correctly
        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        assert_ne!(user.password_hash, Vec::<u8>::new());
    }
}
