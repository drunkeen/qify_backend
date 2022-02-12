use crate::schema::room;
use actix_web::web::Data;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::models;
use crate::models::spotify_id::NewSpotifyUser;
use crate::models::{GenericOutput, ServiceResult};
use crate::schema;

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[table_name = "room"]
pub struct Room {
    pub room_id: String,
    pub spotify_id: String,
    pub room_id_short: String,
}

pub fn get_all_rooms(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
) -> ServiceResult<Vec<Room>> {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::room::table.load::<Room>(&connection)?;

    Ok(models::GenericOutput {
        data: Some(res),
        status_code: 200,
        success: true,
        error: None,
    })
}

pub fn create_room(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    spotify_user: &NewSpotifyUser,
) -> ServiceResult<Room> {
    let connection = pool.get().expect("Could not create connection");

    let existing_rooms = room::table
        .filter(room::spotify_id.eq(&spotify_user.spotify_id))
        .load::<Room>(&connection)?;
    if !existing_rooms.is_empty() {
        // A rooms for spotify_id already exists
        return Ok(GenericOutput {
            error: None,
            data: Some(existing_rooms.into_iter().next().unwrap()),
            success: true,
            status_code: 200,
        });
    }

    // Check as many room_id as needed
    loop {
        let room_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
        let room_id_short = &room_id[0..6];

        let results = diesel::insert_into(room::dsl::room)
            .values(vec![Room {
                spotify_id: spotify_user.spotify_id.clone(),
                room_id: room_id.clone(),
                room_id_short: String::from(room_id_short),
            }])
            .get_results::<Room>(&connection);

        // A room with `room_id_short` already exists
        if let Err(_) = results {
            println!("Room '{}' already exists", &room_id_short);
            continue;
        }

        let result = results.unwrap().into_iter().next().unwrap();
        return Ok(GenericOutput {
            error: None,
            data: Some(result),
            success: true,
            status_code: 200,
        });
    }
}
