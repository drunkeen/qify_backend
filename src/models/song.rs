use actix_web::web::Data;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;
use serde::{Deserialize, Serialize};

use crate::models::ServiceResult;
use crate::schema;
use crate::schema::song;

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Song {
    pub id: i32,

    pub uri: String,
    pub artist: String,
    pub title: String,

    pub room_id: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Insertable, AsChangeset, Debug)]
#[table_name = "song"]
pub struct NewSong {
    pub uri: String,
    pub artist: String,
    pub title: String,

    pub room_id: String,
}

pub fn get_all_songs(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    room_id: String,
) -> ServiceResult<Vec<Song>> {
    use schema::song::dsl;

    let connection = pool.get().expect("Could not create connection");
    let results = schema::song::table
        .filter(dsl::room_id.eq(room_id))
        .load::<Song>(&connection)?;

    Ok(results)
}

pub fn add_song(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    song: NewSong,
) -> ServiceResult<Vec<Song>> {
    use schema::song::dsl;
    let connection = pool.get().expect("Could not create connection");

    let results = diesel::insert_into(dsl::song)
        .values(&song)
        .get_results::<Song>(&connection)?;

    Ok(results)
}
