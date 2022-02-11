use crate::schema::spotify;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize)]
pub struct SpotifyUser {
    pub id: i32,
    pub spotify_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_date: std::time::SystemTime,
}

#[allow(dead_code)]
#[derive(Deserialize, Insertable)]
#[table_name = "spotify"]
pub struct NewSpotifyUser {
    pub spotify_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_date: std::time::SystemTime,
}
