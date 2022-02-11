use crate::schema::room;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[table_name = "room"]
pub struct Room {
    pub room_id: String,
    pub spotify_id: String,
    pub room_id_short: String,
}
