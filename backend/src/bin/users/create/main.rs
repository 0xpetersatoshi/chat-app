use backend::database::establish_connection;
use backend::handlers::users::{create_user, get_user_by_email};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Please provide email, username, and password as command line arguments");
    }

    let user_email = &args[1];
    let username = &args[2];
    let password = &args[3];

    let conn = &mut establish_connection();
    let new_user = create_user(conn, username, user_email, password);
    println!(
        "Created new user {} with ID={}",
        new_user.username, new_user.id
    );
    let user = match get_user_by_email(conn, &new_user.email) {
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
