use crate::models::users::{NewUser, User, UserChanges};
use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_user(
    conn: &mut PgConnection,
    new_username: &str,
    new_email: &str,
    new_password: &str,
) -> User {
    let new_user = NewUser::new(new_username, new_email, new_password);
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error adding new user")
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> QueryResult<usize> {
    let result = diesel::delete(users.find(user_id)).execute(conn)?;

    Ok(result)
}

pub fn update_user(
    conn: &mut PgConnection,
    user_id: i32,
    username_change: Option<String>,
    email_change: Option<String>,
    password_hash_change: Option<Vec<u8>>,
) -> QueryResult<usize> {
    let changes = UserChanges {
        username: username_change,
        email: email_change,
        password_hash: password_hash_change,
    };

    let result = diesel::update(users.find(user_id))
        .set(&changes)
        .execute(conn)?;

    Ok(result)
}

pub fn get_user_by_email_or_username(
    conn: &mut PgConnection,
    email_or_username: &str,
) -> Result<Option<User>, Error> {
    let user = users
        .filter(email.eq(email_or_username).or(username.eq(email_or_username)))
        .limit(1)
        .load::<User>(conn)?
        .pop();

    Ok(user)
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> Result<Option<User>, Error> {
    let user = users
        .filter(id.eq(user_id))
        .limit(1)
        .load::<User>(conn)?
        .pop();

    Ok(user)
}
