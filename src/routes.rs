use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use serde::Serialize;
use std::ops::Add;
use std::time::Duration;

use crate::models;
use crate::models::room::create_room;
#[cfg(debug_assertions)]
use crate::models::room::get_all_rooms;
use crate::models::spotify_api::Code;
#[cfg(debug_assertions)]
use crate::models::spotify_id::get_all_accounts;
use crate::models::spotify_id::{create_spotify_id, NewSpotifyUser};
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};
use crate::spotify_api::{api_spotify_authenticate, api_spotify_me};
use crate::utils::format_error;

fn send_data<T: Serialize>(body: T) -> HttpResponse {
    let data = GenericOutput {
        data: Some(body),
        error: None,
        success: true,
        status_code: 200,
    };

    if let Ok(result) = serde_json::to_string(&data) {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(result)
    } else {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::SERDE_ERROR)
    }
}

fn send_error(
    error: Box<dyn std::error::Error>,
    status_code: u16,
    error_text: &'static str,
) -> HttpResponse {
    let status = StatusCode::from_u16(status_code);
    if let Err(_) = status {
        eprintln!("Status code should always be a valid status code");
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::INTERNAL_SERVER_ERROR);
    }

    let status = status.unwrap();
    let error = format_error(error, error_text);

    let data = GenericOutput::<u8> {
        data: None,
        error: Some(error),
        success: false,
        status_code,
    };

    if let Ok(result) = serde_json::to_string(&data) {
        HttpResponse::build(status)
            .content_type("application/json")
            .body(result)
    } else {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::SERDE_ERROR)
    }
}

pub async fn hello(_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(
    _pool: Data<Pool<ConnectionManager<PgConnection>>>,
    req_body: String,
) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[cfg(debug_assertions)]
#[get("/rooms")]
pub async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let rooms = get_all_rooms(&pool);
    if let Err(error) = rooms {
        return send_error(error, 500, "Rooms: Could not retrieve any room");
    }
    return send_data(rooms.unwrap());
}

#[cfg(debug_assertions)]
#[get("/accounts")]
pub async fn accounts(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let accounts = get_all_accounts(&pool);
    if let Err(error) = accounts {
        return send_error(error, 500, "Accounts: Could not retrieve any account");
    }
    return send_data(accounts.unwrap());
}

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
    if spotify_user.product != String::from("premium") {
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

    let spotify_id = create_spotify_id(&pool, &new_spotify_user);
    if let Err(error) = spotify_id {
        return send_error(error, 500, "Create account: Could not add user");
    }

    let room = create_room(&pool, &new_spotify_user);
    if let Err(error) = room {
        return send_error(error, 500, "Create room: Could not create a new room");
    }

    send_data(room.unwrap())
}
