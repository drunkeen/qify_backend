use actix_web::{
    get, HttpResponse, post, Responder,
};
use actix_web::web::Data;
use awc::http::StatusCode;
use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

use crate::models::Room;
use crate::schema;

#[get("/hello")]
pub async fn hello(_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(debug_assertions)]
#[get("/rooms")]
pub async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::room::table.load::<Room>(&connection);

    if let Ok(result) = res {
        let tmp = serde_json::to_string(&result).unwrap_or(String::from("Empty vec"));
        HttpResponse::Ok().content_type("application/json").status(StatusCode::OK).body(format!("{:?}", tmp))
    } else {
        HttpResponse::Ok().content_type("application/json").status(StatusCode::INTERNAL_SERVER_ERROR).body("[]")
    }
}

#[cfg(not(debug_assertions))]
#[get("/rooms")]
async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").status(StatusCode::NOT_FOUND).body("Not available in release mode")
}

#[post("/echo")]
pub async fn echo(_pool: Data<Pool<ConnectionManager<PgConnection>>>, req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello(_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
