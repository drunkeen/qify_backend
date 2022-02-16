mod models;
mod routes;
mod schema;
mod spotify_api;
mod utils;
mod websocket;

#[macro_use]
extern crate diesel;

use std::env;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use r2d2::Pool;

fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    // let connection = establish_connection();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    dotenv().ok();

    let pool = create_pool();
    HttpServer::new(move || {
        let app = App::new();
        let app = app.data(pool.clone()).service(crate::routes::echo);

        #[cfg(debug_assertions)]
        let app = app
            .service(crate::routes::rooms)
            .service(crate::routes::accounts);

        let app = app.service(crate::routes::spotify_authenticate);
        let app = app
            .route("/hey", web::get().to(crate::routes::hello))
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(crate::websocket::ws_index)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"));

        app
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
