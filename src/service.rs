use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;

use crate::models;
use crate::models::{GenericOutput, Room};
use crate::schema;

type ServiceResult<T> = Result<GenericOutput<T>, Box<dyn std::error::Error>>;

pub fn get_all_rooms(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
) -> ServiceResult<Vec<Room>> {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::room::table.load::<models::Room>(&connection)?;

    Ok(models::GenericOutput {
        data: Some(res),
        status_code: 200,
        success: true,
        error: None,
    })
}

pub fn create_room(pool: &Data<Pool<ConnectionManager<PgConnection>>>) -> ServiceResult<u8> {
    let _connection = pool.get().expect("Could not create connection");

    Ok(GenericOutput {
        error: None,
        data: None,
        success: false,
        status_code: 501,
    })
}

pub fn add_tokens(_pool: &Data<Pool<ConnectionManager<PgConnection>>>) -> ServiceResult<u8> {
    Ok(GenericOutput {
        error: None,
        data: None,
        success: false,
        status_code: 501,
    })
}
