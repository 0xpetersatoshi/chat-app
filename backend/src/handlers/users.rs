use crate::models::users::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_user(
    new_username: &str,
    new_email: &str,
    new_password: &str,
    conn: &mut PgConnection,
) {
    let new_user = NewUser::new(new_username, new_email, new_password);
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error adding new user");

    println!("New user {} added!", new_username)
}

// delete user

// update user

// get user
pub fn get_user_by_email(
    connection: &mut PgConnection,
    user_email: &str,
) -> Result<Option<User>, Error> {
    let user = users
        .filter(email.eq(user_email))
        .limit(1)
        .load::<User>(connection)?
        .pop();

    Ok(user)
}

pub fn get_user_by_id(connection: &mut PgConnection, user_id: i32) -> Result<Option<User>, Error> {
    let user = users
        .filter(id.eq(user_id))
        .limit(1)
        .load::<User>(connection)?
        .pop();

    Ok(user)
}
