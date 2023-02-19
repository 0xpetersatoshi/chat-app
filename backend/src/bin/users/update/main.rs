use backend::database::establish_connection;
use backend::repository::users::{get_user_by_email, get_user_by_username, update_user};
use backend::models::users::User;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Please provide update field (`username` or `email`), `old_value`, `new_value` as command line arguments");
    }

    let update_field = &args[1];
    let old_value = &args[2];
    let new_value = &args[3];

    let pool = establish_connection();
    let mut conn = pool.get().unwrap();
    let user: User;

    if update_field == "email" {
        user = match get_user_by_email(&mut conn, old_value) {
            Ok(Some(u)) => u,
            Ok(None) => panic!("No user with username {} was found", old_value),
            Err(e) => panic!("Error: {}", e),
        };
    } else {
        user = match get_user_by_username(&mut conn, old_value) {
            Ok(Some(u)) => u,
            Ok(None) => panic!("No user with username {} was found", old_value),
            Err(e) => panic!("Error: {}", e),
        };
    }

    let result: Result<(), String> =
        match update_user(&mut conn, user.id, Some(new_value.clone()), None, None) {
            Ok(_) => Ok(()),
            Err(e) => panic!("Error: {}", e),
        };

    match result {
        Ok(_) => println!("User updated successfully"),
        Err(e) => println!("Error updating user: {}", e),
    }
}
