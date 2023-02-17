use backend::handlers::users::{create_user, get_user_by_email};
use backend::database::establish_connection;

fn main() {
    let user_email = "user@myemail.com";
    let conn = &mut establish_connection();
    create_user("user", user_email, "password", conn);
    let user = match get_user_by_email(conn, user_email) {
        Ok(Some(user)) => user,
        Ok(None) => {
            panic!("User not found");
        }
        Err(e) => {
            panic!("Error getting user: {}", e);
        }
    };
    println!("User {:?}", user);
}