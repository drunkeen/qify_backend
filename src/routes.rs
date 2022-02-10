use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use std::ops::Add;
use std::time::Duration;

use crate::models;
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};

use crate::service::{create_room, create_spotify_id, get_all_accounts, get_all_rooms};
use crate::spotify_api::{api_spotify_authenticate, api_spotify_me};

use crate::models::NewSpotifyUser;
use serde::Serialize;

fn respond<T: Serialize>(data: T) -> HttpResponse {
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

fn res_or_error<T: Serialize>(
    data: Result<GenericOutput<T>, Box<dyn std::error::Error>>,
    error_message: &str,
) -> GenericOutput<T> {
    match data {
        Ok(data) => data,
        Err(error) => GenericOutput {
            #[cfg(debug_assertions)]
            error: Some(format!("{}, error: {}", error_message, error)),
            #[cfg(not(debug_assertions))]
            error: Some(String::from(error_message)),
            data: None,
            success: false,
            status_code: 500,
        },
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

#[get("/rooms")]
pub async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    #[cfg(debug_assertions)]
    return respond(res_or_error(
        get_all_rooms(&pool),
        "Rooms: Could not retrieve any room",
    ));

    #[cfg(not(debug_assertions))]
    return HttpResponse::Forbidden()
        .content_type("text/json")
        .body(NOT_IMPLEMENTED_RELEASE_MODE);
}

#[get("/accounts")]
pub async fn accounts(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    #[cfg(debug_assertions)]
    return respond(res_or_error(
        get_all_accounts(&pool),
        "Rooms: Could not retrieve any room",
    ));

    #[cfg(not(debug_assertions))]
    return HttpResponse::Forbidden()
        .content_type("text/json")
        .body(NOT_IMPLEMENTED_RELEASE_MODE);
}

#[post("/spotifyAuthenticate")]
pub async fn spotify_authenticate(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    info: web::Json<models::Code>,
) -> impl Responder {
    let body = info.0;
    let tokens = api_spotify_authenticate(body.code).await;
    let data = res_or_error(
        tokens,
        "Spotify Auth: Could not retrieve tokens from spotify",
    );

    if !&data.success {
        return respond(data);
    }

    let spotify_tokens = data.data.as_ref().unwrap();
    let timestamp =
        std::time::SystemTime::now().add(Duration::from_secs(spotify_tokens.expires_in as u64));

    let spotify_user = api_spotify_me(spotify_tokens.access_token.clone()).await;
    if let Err(error) = spotify_user {
        eprintln!("spotify_authenticate: me: {}", error);
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::INTERNAL_SERVER_ERROR);
    }

    let spotify_user = spotify_user.unwrap();
    let spotify_user = spotify_user.data.unwrap();

    if spotify_user.product != String::from("premium") {
        eprintln!(
            "spotify_authenticate: premium: {} is not premium: {}",
            spotify_user.id, spotify_user.product
        );
        return HttpResponse::Forbidden()
            .content_type("application/json")
            .body(models::SPOTIFY_API_FORBIDDEN);
    }
    let new_spotify_user = NewSpotifyUser {
        spotify_id: spotify_user.id,
        access_token: spotify_tokens.access_token.clone(),
        refresh_token: spotify_tokens.refresh_token.clone(),
        expire_date: timestamp,
    };

    let spotify_id = create_spotify_id(&pool, &new_spotify_user);
    if let Err(error) = spotify_id {
        eprintln!("spotify_authenticate: db: {}", error);
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::INTERNAL_SERVER_ERROR);
    }

    let spotify_id = create_room(&pool, &new_spotify_user);
    if let Err(error) = spotify_id {
        eprintln!("spotify_authenticate: db: {}", error);
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::INTERNAL_SERVER_ERROR);
    }

    respond(data)
}
