use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use crate::models;
#[cfg(debug_assertions)]
use crate::models::room::clear_rooms;
#[cfg(debug_assertions)]
use crate::models::room::get_all_rooms;
use crate::models::room::get_one_room;
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};
use crate::routes::{send_data, send_error};

#[cfg(debug_assertions)]
#[get("/rooms")]
pub async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let rooms = get_all_rooms(&pool);
    if let Err(error) = rooms {
        return send_error(error, 500, "Rooms: Could not retrieve any room");
    }
    return send_data(rooms.unwrap());
}

#[cfg(debug_assertions)]
#[post("/resetRooms")]
pub async fn reset_rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let res = clear_rooms(&pool);
    match res {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(_) => HttpResponse::InternalServerError().body(models::INTERNAL_SERVER_ERROR),
    }
}
