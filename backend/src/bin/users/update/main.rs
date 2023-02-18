use backend::database::establish_connection;
use backend::handlers::users::{get_user_by_email, get_user_by_username, update_user};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Please provide update field (`username` or `email`), `old_value`, `new_value` as command line arguments");
    }

    let update_field = &args[1];
    let old_value = &args[2];
    let new_value = &args[3];

    let conn = &mut establish_connection();
    let result: Result<(), String>;

    if update_field == "email" {
        let user = match get_user_by_email(conn, old_value) {
            Ok(Some(u)) => u,
            Ok(None) => {
                panic!("No user with username {} was found", old_value);
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        result = match update_user(conn, user.id, None, Some(new_value.clone()), None) {
            Ok(_) => Ok(()),
            Err(e) => {
                panic!("Error: {}", e);
            }
        };
    } else {
        let user = match get_user_by_username(conn, old_value) {
            Ok(Some(u)) => u,
            Ok(None) => {
                panic!("No user with username {} was found", old_value);
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        result = match update_user(conn, user.id, Some(new_value.clone()), None, None) {
            Ok(_) => Ok(()),
            Err(e) => {
                panic!("Error: {}", e);
            }
        };
    }

    match result {
        Ok(_) => println!("User updated successfully"),
        Err(e) => println!("Error updating user: {}", e),
    }
}
