use argon2::{self, Config};
use chrono::NaiveDateTime;
use diesel::prelude::*;

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
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: Vec<u8>,
}

impl<'a> NewUser<'a> {
    pub fn new(username: &'a str, email: &'a str, password: &'a str) -> Self {
        let config = Config::default();
        let password_hash = argon2::hash_encoded(password.as_bytes(), b"", &config).unwrap();
        let password_hash = password_hash.as_bytes().to_vec();

        NewUser {
            username,
            email,
            password_hash,
        }
    }
}
