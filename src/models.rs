use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Queryable, Serialize, Deserialize)]
pub struct Room {
    pub room_id: String,
    pub spotify_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenericOutput<T> {
    pub error: Option<String>,
    pub success: bool,
    pub status_code: u16,
    pub data: Option<T>,
}

pub const SERDE_ERROR: &str =
    "{\"data\":null,\"success\":false,\"status_code\":500,\"error\":\"JSON: Error converting to string\"}";
