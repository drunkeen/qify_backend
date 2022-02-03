use std::env;

use actix_web::web::Data;
use actix_web::{get, post, HttpResponse, Responder};
use awc::http::StatusCode;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;

use crate::models::Room;
use crate::schema;

pub fn get_all_rooms(pool: &Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::room::table.load::<Room>(&connection);

    if let Ok(result) = res {
        if let Ok(result) = serde_json::to_string(&result) {
            HttpResponse::Ok()
                .content_type("application/json")
                .status(StatusCode::OK)
                .body(format!("{:?}", result))
        } else {
            HttpResponse::Ok()
                .content_type("application/json")
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{:?}", "[]"))
        }
    } else {
        HttpResponse::Ok()
            .content_type("application/json")
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("[]")
    }
}
