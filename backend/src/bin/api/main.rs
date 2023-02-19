use actix_web::{App, HttpServer, web};
use backend::database::establish_connection;
use backend::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();
    let data = web::Data::new(pool);

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(handlers::users::get_user_by_id)
            .service(handlers::users::get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
