use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;
use rand::{distributions::Alphanumeric, Rng};

use diesel::prelude::*;

use crate::models;
use crate::models::room::Room;
use crate::models::spotify_id::{NewSpotifyUser, SpotifyUser};
use crate::models::GenericOutput;
use crate::schema;

type ServiceResult<T> = Result<GenericOutput<T>, Box<dyn std::error::Error>>;

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

pub fn get_all_accounts(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
) -> ServiceResult<Vec<SpotifyUser>> {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::spotify::table.load::<SpotifyUser>(&connection)?;

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
    use crate::schema::room;
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

    loop {
        // Check as many room_id as needed
        let room_id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
        let room_id_short = String::from(room_id.clone().split_at(6).0);

        let results = diesel::insert_into(room::dsl::room)
            .values(vec![Room {
                spotify_id: spotify_user.spotify_id.clone(),
                room_id: room_id.clone(),
                room_id_short: room_id_short.clone(),
            }])
            .get_results::<Room>(&connection);

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

pub fn create_spotify_id(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    spotify_user: &NewSpotifyUser,
) -> ServiceResult<Vec<SpotifyUser>> {
    let connection = pool.get().expect("Could not create connection");
    let results = diesel::insert_into(schema::spotify::dsl::spotify)
        .values(vec![spotify_user])
        .on_conflict_do_nothing()
        .get_results::<SpotifyUser>(&connection)?;

    Ok(GenericOutput {
        error: None,
        data: Some(results),
        success: true,
        status_code: 200,
    })
}
