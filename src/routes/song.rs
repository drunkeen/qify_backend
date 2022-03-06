use actix_web::web::Data;
use actix_web::{get, post, web, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::models::song::get_all_songs;
use crate::models::song::{add_song, NewSong};
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};
use crate::routes::{send_data, send_error, StringError};
use crate::utils::{RoomAction, RoomData};

#[derive(Serialize, Deserialize, Debug)]
struct AddSongProps {
    pub title: String,
    pub duration_ms: i32,
    pub image: String,
    pub album: String,
    pub uri: String,
}

#[get("/songs/{room_id}")]
pub async fn get_songs(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    latest_inserts: Data<Mutex<HashMap<String, RoomData>>>,
    web::Path(room_id): web::Path<String>,
) -> impl Responder {
    let mut latest_inserts = latest_inserts.lock().unwrap();
    if let Some(room_latest) = latest_inserts.get_mut(&room_id) {
        (*room_latest).update(None, Some(RoomAction::SongSearch));
    }

    let songs = get_all_songs(&pool, room_id);
    if let Err(error) = songs {
        return send_error(error, 500, "GetSongs: Could not retrieve any song");
    }

    send_data(songs.unwrap())
}

#[post("/songs/{room_id}")]
pub async fn add_songs(
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    latest_inserts: Data<Mutex<HashMap<String, RoomData>>>,
    web::Path(room_id): web::Path<String>,
    data: String,
) -> impl Responder {
    let data = serde_json::from_str::<AddSongProps>(&*data);
    if let Err(error) = data {
        const ERROR: &str = "AddSongs: Data should contain fields room_id and title";
        return send_error(error.into(), 500, ERROR);
    }

    let mut latest_inserts = latest_inserts.lock().unwrap();

    let data = data.unwrap();
    if let Some(latest) = latest_inserts.get(&room_id) {
        if latest.uri == data.uri {
            return send_error(
                Box::new(StringError("Logic Error")),
                500,
                "AddSongs: Trying to add the last song again",
            );
        }
    }

    if let Some(room_latest) = latest_inserts.get_mut(&room_id) {
        (*room_latest).update(Some(data.uri.clone()), Some(RoomAction::SongAdd));
    }

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
