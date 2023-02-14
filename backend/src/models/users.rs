use argon2::{self, Config};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: Vec<u8>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
        let password_hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();
        let password_hash = password_hash.as_bytes().to_vec();

        NewUser {
            username,
            email,
            password_hash,
        }
    }
}
