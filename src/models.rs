use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize)]
pub struct Room {
    pub room_id: String,
    pub spotify_id: String,
}
