use actix_web::web::Data;
use actix_web::{get, post, web, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use serde::{Deserialize, Serialize};

use crate::models::action::push_actions;
use crate::models::room::{get_latest_track, set_latest_track};
use crate::models::song::get_all_songs;
use crate::models::song::{add_song, NewSong};
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};
use crate::routes::{send_data, send_error, StringError};

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
    web::Path(room_id): web::Path<String>,
) -> impl Responder {
    let add = push_actions(&pool, room_id.clone(), String::from("SongGet"));
    if let Err(error) = add {
        dbg!(&error);
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
    web::Path(room_id): web::Path<String>,
    data: String,
) -> impl Responder {
    let add = push_actions(&pool, room_id.clone(), String::from("AddSong"));
    if let Err(error) = add {
        dbg!(&error);
    }

    let data = serde_json::from_str::<AddSongProps>(&*data);
    if let Err(error) = data {
        const ERROR: &str = "AddSongs: Data should contain fields room_id and title";
        return send_error(error.into(), 500, ERROR);
    }

    let latest_track = get_latest_track(&pool, room_id.clone());
    dbg!(&latest_track);
    if let Err(error) = latest_track {
        const ERROR: &str = "AddSongs: Can't get latest_track";
        return send_error(error, 500, ERROR);
    }

    let latest_track = latest_track.unwrap();

    let data = data.unwrap();
    if *latest_track == *data.uri {
        return send_error(
            Box::new(StringError("Logic Error")),
            500,
            "AddSongs: Trying to add the last song again",
        );
    }

    let add = add_song(
        &pool,
        NewSong {
            room_id: room_id.clone(),
            title: data.title,
            uri: data.uri.clone(),
            album: data.album,
            duration_ms: data.duration_ms,
            image: data.image,
        },
    );

    if let Err(error) = add {
        return send_error(error, 500, "AddSongs: Could not add song to room");
    }

    let _ = set_latest_track(&pool, room_id, data.uri);

    send_data(add.unwrap())
}
