use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;
use serde::{Deserialize, Serialize};

use crate::models::{GenericOutput, ServiceResult};
use crate::schema;
use crate::schema::spotify;

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize)]
pub struct SpotifyUser {
    pub id: i32,
    pub spotify_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_date: std::time::SystemTime,
}

#[allow(dead_code)]
#[derive(Deserialize, Insertable)]
#[table_name = "spotify"]
pub struct NewSpotifyUser {
    pub spotify_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_date: std::time::SystemTime,
}

pub fn get_all_accounts(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
) -> ServiceResult<Vec<SpotifyUser>> {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::spotify::table.load::<SpotifyUser>(&connection)?;

    Ok(GenericOutput {
        data: Some(res),
        status_code: 200,
        success: true,
        error: None,
    })
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
