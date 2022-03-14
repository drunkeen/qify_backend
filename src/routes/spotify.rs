use actix_web::web::Data;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use std::env;
use std::ops::Add;
use std::time::Duration;

use crate::models::action::push_actions;
use crate::models::room::create_room;
use crate::models::spotify_api::{Code, SpotifySearchResult, SpotifyTrackFiltered};
#[cfg(debug_assertions)]
use crate::models::spotify_id::get_all_accounts;
use crate::models::spotify_id::{create_spotify_id, get_one_account, NewSpotifyUser};
#[allow(unused_imports)]
use crate::models::{GenericOutput, NOT_IMPLEMENTED_RELEASE_MODE};
use crate::routes::{send_data, send_error};
use crate::spotify_api::{api_spotify_authenticate, api_spotify_me, api_spotify_search};

#[derive(Deserialize, Serialize, Debug)]
pub struct SearchProps {
    pub q: String,
    pub offset: u16,
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

    let res = room.unwrap();

    let spotify_id = create_spotify_id(&pool, &new_spotify_user);
    if let Err(error) = spotify_id {
        return send_error(error, 500, "Create account: Could not add user");
    }

    let add = push_actions(&pool, res.room_id.clone(), String::from("RoomCreate"));
    if let Err(error) = add {
        dbg!(&error);
    }

    send_data(res)
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

#[get("/search/{room_id}")]
pub async fn search(
    req: HttpRequest,
    pool: Data<Pool<ConnectionManager<PgConnection>>>,
    web::Path(room_id): web::Path<String>,
) -> impl Responder {
    let add = push_actions(&pool, room_id.clone(), String::from("SearchSong"));
    if let Err(error) = add {
        dbg!(&error);
    }

    // Hack to remove % decoding
    let query_string = req.query_string().replace('%', "%25");
    let query_params = serde_urlencoded::from_str::<SearchProps>(&query_string);
    if let Err(error) = query_params {
        return send_error(error.into(), 400, "Search: Missing fields");
    }

    let info = query_params.unwrap();
    if info.q.is_empty() {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .body(crate::models::SPOTIFY_API_SEARCH_MISSING_FIELDS);
    }

    let account = get_one_account(&pool, &room_id);
    if let Err(error) = account {
        const ERROR: &str = "Search: Could not associate room_id to spotify_id";
        return send_error(error, 500, ERROR);
    }

    let account = account.unwrap();

    let search_results = api_spotify_search(&account.access_token, &info.q, info.offset).await;
    if let Err(error) = search_results {
        return send_error(error, 500, "Search: Could not fetch data from Spotify");
    }

    let url = env::var("BACKEND_URL").unwrap_or_else(|_| String::from("http://0.0.0.0:8080"));

    let tracks = search_results.unwrap().tracks;
    let filtered_items = tracks
        .items
        .into_iter()
        .map(|t| SpotifyTrackFiltered {
            image: t.album.images.into_iter().map(|i| i.url).next(),
            uri: t.uri,
            album: t.album.name,
            title: t.name,
            duration_ms: t.duration_ms,
        })
        .collect::<Vec<_>>();

    let filtered_search = SpotifySearchResult::<SpotifyTrackFiltered> {
        items: filtered_items,
        offset: tracks.offset,
        next: if tracks.next.is_some() {
            Some(format!(
                "{}/search/{}?q={}&offset={}",
                &url,
                &room_id,
                &info.q,
                &info.offset + 10
            ))
        } else {
            None
        },
        limit: tracks.limit,
        previous: None,
        total: tracks.total,
    };

    send_data(filtered_search)
}
