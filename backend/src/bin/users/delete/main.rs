use backend::database::establish_connection;
use backend::repository::users::{delete_user, get_user_by_email, get_user_by_username};
use backend::models::users::User;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide field type (`username` or `email`), `field value` as command line arguments");
    }

    let field_type = &args[1];
    let field = &args[2];

    let pool = establish_connection();
    let mut conn = pool.get().unwrap();
    let user: User;

    if field_type == "email" {
        user = match get_user_by_email(&mut conn, field) {
            Ok(Some(u)) => u,
            Ok(None) => panic!("No user found"),
            Err(e) => panic!("Error: {}", e),
        };
    } else {
        user = match get_user_by_username(&mut conn, field) {
            Ok(Some(u)) => u,
            Ok(None) => panic!("No user found"),
            Err(e) => panic!("Error: {}", e),
        };
    }

    let result: Result<(), String> = match delete_user(&mut conn, user.id) {
        Ok(_) => Ok(()),
        Err(e) => panic!("Error: {}", e),
    };

    match result {
        Ok(_) => println!("Successfully deleted user {}", field),
        Err(e) => panic!("Error: {}", e),
    }
}
