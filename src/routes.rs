use actix_web::web::Data;
use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use serde_json::json;

use crate::models;
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};

use crate::service::get_all_rooms;
use crate::spotify_api::api_spotify_authenticate;

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
    return get_all_rooms(&pool);

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

    match tokens {
        Ok(tokens) => HttpResponse::Ok().content_type("text/json").json(tokens),
        Err(_) => HttpResponse::InternalServerError()
            .content_type("text/json")
            .json(json!(GenericOutput::<u8> {
                data: None,
                status_code: 500,
                success: false,
                error: Some("Spotify Auth: Could not retrieve tokens")
            })),
    }
}
