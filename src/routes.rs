use actix_web::web::Data;
use actix_web::{get, post, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use crate::service::get_all_rooms;

pub async fn hello(_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(
    _pool: Data<Pool<ConnectionManager<PgConnection>>>,
    req_body: String,
) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/rooms")]
pub async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    #[cfg(debug_assertions)]
    return get_all_rooms(&pool);

    #[cfg(not(debug_assertions))]
    return HttpResponse::Forbidden()
        .content_type("text/plain")
        .body("Not available in release mode");
}
