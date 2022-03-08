mod models;
mod routes;
mod schema;
mod spotify_api;
mod utils;
mod websocket;

#[macro_use]
extern crate diesel;

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use std::time::Duration;

#[cfg(debug_assertions)]
use crate::models::room::get_all_rooms;
use actix_files as fs;
// use actix_web::middleware;
use crate::utils::{RoomData, ROOM_ACTION_DEFAULT};
use actix_web::dev::Service;
#[cfg(debug_assertions)]
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv::dotenv;
use futures::FutureExt;
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
    let pool_clone = pool.clone();

    actix_rt::spawn(async move {
        const DELAY: Duration = Duration::from_secs(60 * 30);
        let mut interval = actix_rt::time::interval(DELAY);
        loop {
            interval.tick().await;

            // Removes old rooms
            let _ = crate::models::room::clear_old_rooms(&pool_clone);
            // Refresh all spotify accounts
            let _ = crate::models::spotify_id::refresh_all_accounts(&pool_clone).await;
        }
    });

    // Key: RoomId / Value: song uri

    let rooms = crate::models::room::get_all_rooms(&Data::new(pool.clone())).unwrap();
    let latest_insert = rooms
        .into_iter()
        .map(|r| (r.room_id, ROOM_ACTION_DEFAULT))
        .collect::<HashMap<_, _>>();

    #[cfg(debug_assertions)]
    {
        let rooms = get_all_rooms(&Data::new(pool.clone())).unwrap();
        println!("There still is {} rooms left.\n", rooms.len());
        println!("{:?}\n", rooms);
    }

    HttpServer::new(move || {
        let app = App::new();
        // let app = app.wrap(middleware::Logger::default());
        #[cfg(debug_assertions)]
        let app = app.wrap_fn(|req, srv| {
            println!("{}\t{}", req.method(), req.path());
            srv.call(req).map(|res| res)
        });

        // Adds database connection pool to all routes
        let app = app.data(pool.clone());
        let app = app.data(Mutex::new(latest_insert.clone()));

        // Adds routes avail. only in debug
        #[cfg(debug_assertions)]
        let app = app
            .service(crate::routes::room::rooms)
            .service(crate::routes::spotify::accounts)
            .service(crate::routes::room::reset_rooms);

        // Song
        let app = app
            .service(crate::routes::song::get_songs)
            .service(crate::routes::song::add_songs);

        // Room
        let app = app.service(crate::routes::room::room);

        // Spotify
        let app = app
            .service(crate::routes::spotify::spotify_authenticate)
            .service(crate::routes::spotify::search);

        app
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(crate::websocket::ws_index)))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
