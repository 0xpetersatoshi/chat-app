use crate::models::users::{NewUser};
use crate::schema::users;
use diesel::prelude::*;

pub fn create_user(username: &str, email: &str, password: &str, conn: &mut PgConnection) {
    let new_user = NewUser::new(username, email, password);
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error adding new user");

    println!("New user {} added!", username)
}
