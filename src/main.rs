mod models;
mod routes;
mod schema;
mod spotify_api;
mod utils;
mod websocket;

#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use std::time::Duration;

use actix_files as fs;
// use actix_web::middleware;
use crate::utils::create_pool;
#[cfg(debug_assertions)]
use actix_web::{dev::Service, web::Data};
use actix_web::{web, App, HttpServer};
#[cfg(debug_assertions)]
use futures::FutureExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    println!("Waiting for pool");
    let mut pool = create_pool();
    while pool.is_err() {
        println!(
            "{:?}: No pool, trying again...",
            std::time::SystemTime::now()
        );
        pool = create_pool();
    }
    println!("Pool is up");

    let pool = pool.unwrap();

    let pool_clone = pool.clone();

    #[cfg(debug_assertions)]
    {
        let rooms = crate::models::room::get_all_rooms(&Data::new(pool.clone())).unwrap();
        println!("There still is {} rooms left.\n", rooms.len());
        println!("{:?}\n", rooms);
    }

    // Refresh all spotify accounts
    // Fails if DB is not accessible
    let _ = crate::models::spotify_id::refresh_all_accounts(&pool_clone).await;
    actix_rt::spawn(async move {
        const DELAY: Duration = Duration::from_secs(60 * 30);
        let mut interval = actix_rt::time::interval(DELAY);
        loop {
            interval.tick().await;

            // Removes old rooms
            let _ = crate::models::room::clear_old_rooms(&pool_clone);
        }
    });

    // Key: RoomId / Value: song uri
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:19006")
            .allowed_origin("http://127.0.0.1:19006")
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin(&std::env::var("EXPOSE_URL").expect("EXPOSE_URL needs to be set"))
            .allowed_methods(vec!["GET", "POST", "OPTIONS", "DELETE"])
            .allowed_header("Content-Type");

        let app = App::new().wrap(cors);
        // let app = app.wrap(middleware::Logger::default());
        #[cfg(debug_assertions)]
        let app = app.wrap_fn(|req, srv| {
            println!("{}\t{}", req.method(), req.path());
            srv.call(req).map(|res| res)
        });

        // Adds database connection pool to all routes
        let app = app.data(pool.clone());

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
    // start http server on 0.0.0.0:8080
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
