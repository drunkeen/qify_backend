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
#[derive(Deserialize, Insertable, AsChangeset)]
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
    use schema::spotify::dsl;

    let connection = pool.get().expect("Could not create connection");
    let results = diesel::insert_into(dsl::spotify)
        .values(spotify_user)
        .on_conflict(dsl::spotify_id)
        .do_update()
        .set(spotify_user)
        .get_results::<SpotifyUser>(&connection);

    if let Err(error) = &results {
        #[cfg(debug_assertions)]
        return Err(format!("Could not create or update spotify user, error: {}", error).into());
        #[cfg(not(debug_assertions))]
        return Err(format!("Could not create or update spotify user").into());
    }

    Ok(GenericOutput {
        error: None,
        data: Some(results.unwrap()),
        success: true,
        status_code: 200,
    })
}
