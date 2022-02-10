use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;
use serde::Serialize;

use crate::models;
use crate::schema;

fn respond<T: Serialize>(data: T) -> impl Responder {
    if let Ok(result) = serde_json::to_string(&data) {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("{:?}", result))
    } else {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(format!("{:?}", models::SERDE_ERROR))
    }
}

pub fn get_all_rooms(pool: &Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::room::table.load::<models::Room>(&connection);

    let result = if let Ok(result) = res {
        models::GenericOutput {
            data: Some(result),
            status_code: 200,
            success: true,
            error: None,
        }
    } else {
        models::GenericOutput {
            data: None,
            status_code: 500,
            success: false,
            error: Some("ROOMS: Could not connect to database"),
        }
    };

    respond(result)
}
