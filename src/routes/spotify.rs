#[cfg(debug_assertions)]
use actix_web::get;
use actix_web::web::Data;
use actix_web::{post, web, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use std::ops::Add;
use std::time::Duration;

use crate::models::room::create_room;
use crate::models::spotify_api::Code;
#[cfg(debug_assertions)]
use crate::models::spotify_id::get_all_accounts;
use crate::models::spotify_id::{create_spotify_id, NewSpotifyUser};
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};
use crate::routes::{send_data, send_error};
use crate::spotify_api::{api_spotify_authenticate, api_spotify_me};

#[post("/spotifyAuthenticate")]
pub async fn spotify_authenticate(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    info: web::Json<Code>,
) -> impl Responder {
    let body = info.0;
    let tokens = api_spotify_authenticate(body.code).await;

    if let Err(error) = tokens {
        return send_error(error, 500, "Spotify Auth: Could not retrieve tokens");
    }

    let spotify_tokens = tokens.unwrap();

    let (access_token, refresh_token, expires_in) = (
        spotify_tokens.access_token,
        spotify_tokens.refresh_token,
        spotify_tokens.expires_in as u64,
    );
    let timestamp = std::time::SystemTime::now().add(Duration::from_secs(expires_in));

    let spotify_user = api_spotify_me(&access_token).await;
    if let Err(error) = spotify_user {
        return send_error(error, 500, "Spotify me: Could not access user information");
    }

    let spotify_user = spotify_user.unwrap();
    if spotify_user.product != *"premium" {
        return send_error(
            String::from("Account not premium").into(),
            403,
            "Spotify me: Spotify account needs to be premium",
        );
    }

    let new_spotify_user = NewSpotifyUser {
        spotify_id: spotify_user.id,
        access_token,
        refresh_token,
        expire_date: timestamp,
    };

    let room = create_room(&pool, &new_spotify_user);
    if let Err(error) = room {
        return send_error(error, 500, "Create room: Could not create a new room");
    }

    let spotify_id = create_spotify_id(&pool, &new_spotify_user);
    if let Err(error) = spotify_id {
        return send_error(error, 500, "Create account: Could not add user");
    }

    send_data(room.unwrap())
}

#[cfg(debug_assertions)]
#[get("/accounts")]
pub async fn accounts(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let accounts = get_all_accounts(&pool);
    if let Err(error) = accounts {
        return send_error(error, 500, "Accounts: Could not retrieve any account");
    }

    send_data(accounts.unwrap())
}
