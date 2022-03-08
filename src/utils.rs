use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use std::env;

#[cfg(debug_assertions)]
pub fn format_error(error: Box<dyn std::error::Error>, error_message: &'static str) -> String {
    format!("{}, error: {}", error_message, error)
}

#[cfg(not(debug_assertions))]
pub fn format_error(_error: Box<dyn std::error::Error>, error_message: &'static str) -> String {
    error_message.to_string()
}

pub const ROOM_ACTION_DEFAULT: RoomData = RoomData {
    action: RoomAction::Unknown,
    uri: String::new(),
    latest_change: std::time::SystemTime::UNIX_EPOCH,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RoomAction {
    RoomConnect,
    RoomRefresh,
    RoomData,

    SongData,
    SongAdd,
    SongSearch,

    Unknown,
}

#[derive(Debug, Clone)]
pub struct RoomData {
    pub latest_change: std::time::SystemTime,
    pub uri: String,
    pub action: RoomAction,
}

impl RoomData {
    #[allow(dead_code)]
    pub fn new(uri: String, action: RoomAction) -> RoomData {
        RoomData {
            uri,
            action,
            latest_change: std::time::SystemTime::now(),
        }
    }

    pub fn update(&mut self, uri: Option<String>, action: Option<RoomAction>) {
        if let Some(uri) = uri {
            self.uri = uri;
        }
        if let Some(action) = action {
            self.action = action;
        }
        self.latest_change = std::time::SystemTime::now();
    }
}

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    // let connection = establish_connection();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
