use actix_web::web::Data;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::ops::Sub;
use std::time::Duration;

use crate::models::spotify_id::NewSpotifyUser;
use crate::models::ServiceResult;
#[cfg(debug_assertions)]
use crate::schema;
use crate::schema::room;
use crate::utils::format_error;

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[table_name = "room"]
pub struct Room {
    pub room_id: String,
    pub spotify_id: String,
    pub room_id_short: String,
    pub creation_date: std::time::SystemTime,
}

#[cfg(debug_assertions)]
pub fn get_all_rooms(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
) -> ServiceResult<Vec<Room>> {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::room::table.load::<Room>(&connection)?;

    Ok(res)
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
        #[cfg(debug_assertions)]
        println!("Room already exists for user {}", &spotify_user.spotify_id);
        // A rooms for spotify_id already exists
        return Ok(existing_rooms.into_iter().next().unwrap());
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
                creation_date: std::time::SystemTime::now(),
            }])
            .get_results::<Room>(&connection);

        // A room with `room_id_short` already exists
        if let Err(_) = results {
            println!("Room '{}' already exists", &room_id_short);
            continue;
        }

        let result = results.unwrap().into_iter().next().unwrap();
        return Ok(result);
    }
}

fn print_rooms(rooms: Vec<Room>) {
    println!(
        "{:?}: clearing {} rooms: {:?}",
        std::time::SystemTime::now(),
        rooms.len(),
        rooms
            .into_iter()
            .map(|r| r.spotify_id)
            .collect::<Vec<String>>()
    );
}

#[cfg(debug_assertions)]
pub fn clear_rooms(pool: &Pool<ConnectionManager<PgConnection>>) -> ServiceResult<()> {
    use crate::schema::room::dsl;
    let connection = pool.get().expect("Could not create connection");
    let results = diesel::delete(dsl::room).get_results::<Room>(&connection);

    if let Err(error) = results {
        return Err(format_error(error.into(), "test").into());
    }

    let results = results.unwrap();
    print_rooms(results);

    Ok(())
}

pub fn clear_old_rooms(pool: &Pool<ConnectionManager<PgConnection>>) -> ServiceResult<()> {
    use crate::schema::room::dsl;
    const DAY_DURATION: Duration = Duration::from_secs(60 * 60 * 24);

    let connection = pool.get().expect("Could not create connection");

    let now = std::time::SystemTime::now();
    let results = diesel::delete(dsl::room)
        .filter(dsl::creation_date.le(now.sub(DAY_DURATION)))
        .get_results::<Room>(&connection);

    if let Err(error) = results {
        return Err(format_error(error.into(), "test").into());
    }

    let results = results.unwrap();
    print_rooms(results);

    Ok(())
}
