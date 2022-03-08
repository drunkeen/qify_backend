use actix_web::web::Data;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;
use serde::{Deserialize, Serialize};

use crate::models::ServiceResult;
use crate::schema;
use crate::schema::spotify;
use crate::spotify_api::api_spotify_refresh;
use crate::utils::format_error;

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize, Debug)]
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

    Ok(res)
}

pub fn get_one_account(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    room_id_full: &str,
) -> ServiceResult<SpotifyUser> {
    use crate::schema::spotify::dsl;

    let connection = pool.get().expect("Could not create connection");
    let account = schema::spotify::table
        .filter(schema::room::dsl::room_id.eq(room_id_full))
        .inner_join(
            schema::room::table.on(schema::spotify::spotify_id.eq(schema::room::spotify_id)),
        )
        .select((
            dsl::id,
            dsl::spotify_id,
            dsl::access_token,
            dsl::refresh_token,
            dsl::expire_date,
        ))
        .first::<SpotifyUser>(&connection)?;

    Ok(account)
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

    if let Err(error) = results {
        let error = format_error(error.into(), "Could not create or update spotify user");
        return Err(error.into());
    }

    Ok(results.unwrap())
}

pub async fn refresh_all_accounts(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> ServiceResult<Vec<SpotifyUser>> {
    let connection = pool.get().expect("Could not create connection");
    let mut accounts = schema::spotify::table.load::<SpotifyUser>(&connection)?;

    // let mut accounts = accounts
    //     .into_iter()
    //     .map(|mut acc| (acc, api_spotify_refresh(acc.refresh_token.clone())))
    //     .collect::<Vec<_>>();

    for acc in &mut accounts {
        let new_tokens = api_spotify_refresh(acc.refresh_token.clone()).await;
        if let Ok(new_tokens) = new_tokens {
            acc.access_token = new_tokens.access_token;
            println!("Fix one");
        }
    }

    Ok(accounts)
}
