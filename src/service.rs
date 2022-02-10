use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use r2d2::Pool;
use rand::{distributions::Alphanumeric, Rng};

use diesel::prelude::*;

use crate::models;
use crate::models::{GenericOutput, NewSpotifyUser, Room, SpotifyUser};
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

pub fn get_all_accounts(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
) -> ServiceResult<Vec<SpotifyUser>> {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::spotify::table.load::<models::SpotifyUser>(&connection)?;

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
        .load::<models::Room>(&connection)?;
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
            .take(1)
            .map(char::from)
            .collect::<String>()
            .to_uppercase();
        println!("Trying {}", room_id);

        let results = diesel::insert_into(room::dsl::room)
            .values(vec![Room {
                spotify_id: spotify_user.spotify_id.clone(),
                room_id: room_id.clone(),
            }])
            .get_results::<Room>(&connection);

        if let Err(error) = results {
            println!("A room '{}' with said already exists", room_id);
            continue;
        }

        let result = results.unwrap().into_iter().next().unwrap();
        println!("Done: {:?}", &result);
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
