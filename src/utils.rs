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

pub fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    // let connection = establish_connection();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
