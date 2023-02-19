use actix_web::{get, web, HttpResponse, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

use crate::models::users::UserResponse;
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

#[get("/api/v1/users")]
async fn get_all_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Error getting db connection from pool");
    let all_users = users::get_all_users(&mut conn).unwrap();

    let user_response: Vec<UserResponse> = all_users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
        .collect();

    HttpResponse::Ok().json(user_response)
}
