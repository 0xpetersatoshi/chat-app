use backend::handlers::users::create_user;
use backend::database::establish_connection;

fn main() {
    let conn = &mut establish_connection();
    create_user("user", "user@myemail.com", "password", conn)
}