use backend::database::establish_connection;
use backend::handlers::users::{create_user, get_user_by_email_or_username};

fn main() {
    let user_email = "user@myemail.com";
    let username = "myusername";
    let password = "password";

    let conn = &mut establish_connection();
    let new_user = create_user(conn, username, user_email, password);
    println!(
        "Created new user {} with ID={}",
        new_user.username, new_user.id
    );
    let user = match get_user_by_email_or_username(conn, &new_user.email) {
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
