use actix_web::web::Data;
use actix_web::{get, post, web, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use serde::{Deserialize, Serialize};

use crate::models::song::get_all_songs;
use crate::models::song::{add_song, NewSong};
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};
use crate::routes::{send_data, send_error};

#[get("/songs/{room_id}")]
pub async fn get_songs(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    web::Path(room_id): web::Path<String>,
) -> impl Responder {
    let songs = get_all_songs(&pool, room_id);
    if let Err(error) = songs {
        return send_error(error, 500, "GetSongs: Could not retrieve any song");
    }

    send_data(songs.unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
struct Props {
    pub title: String,
    pub duration_ms: i32,
    pub image: String,
    pub album: String,
    pub uri: String,
}

#[post("/songs/{room_id}")]
pub async fn add_songs(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    web::Path(room_id): web::Path<String>,
    data: String,
) -> impl Responder {
    let data = serde_json::from_str::<Props>(&*data);
    if let Err(error) = data {
        return send_error(
            error.into(),
            500,
            "AddSongs: Data should contain fields room_id and title",
        );
    }

    let data = data.unwrap();
    let add = add_song(
        &pool,
        NewSong {
            room_id,
            title: data.title,
            uri: data.uri,
            album: data.album,
            duration_ms: data.duration_ms,
            image: data.image,
        },
    );

    if let Err(error) = add {
        return send_error(error, 500, "AddSongs: Could not add song to room");
    }

    send_data(add.unwrap())
}
