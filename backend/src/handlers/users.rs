use actix_web::{get, web, HttpResponse, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::repository::users;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/api/v1/user/{id}")]
async fn get_user_by_id(pool: web::Data<DbPool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let mut conn = pool.get().expect("Error getting db connection from pool");
    let user = users::get_user_by_id(&mut conn, id).unwrap();

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}
