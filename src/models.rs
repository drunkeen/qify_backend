#[allow(dead_code)]
#[derive(Queryable)]
pub struct Room {
    room_id: diesel::types::Varchar,
    spotify_id: diesel::types::Varchar,
}
