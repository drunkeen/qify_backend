use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

use crate::models;
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};

use crate::service::get_all_rooms;
use crate::spotify_api::api_spotify_authenticate;

use serde::Serialize;

fn respond<T: Serialize>(data: T) -> impl Responder {
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

#[post("/spotifyAuthenticate")]
pub async fn spotify_authenticate(
    _pool: Data<Pool<ConnectionManager<PgConnection>>>,
    info: web::Json<models::Code>,
) -> impl Responder {
    let body = info.0;
    let tokens = api_spotify_authenticate(body.code).await;

    respond(res_or_error(
        tokens,
        "Spotify Auth: Could not retrieve tokens from spotify",
    ))
}
