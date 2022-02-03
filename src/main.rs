mod models;
mod schema;
mod websocket;
mod routes;

#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{
    App, HttpServer, middleware, web,
};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
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
        App::new()
            // enable logger
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(crate::routes::hello)
            .service(crate::routes::echo)
            .service(crate::routes::rooms)
            .route("/hey", web::get().to(crate::routes::manual_hello))
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(crate::websocket::ws_index)))
        // static files
        // .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
        // start http server on 127.0.0.1:8080
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
