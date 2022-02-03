//! Simple echo websocket server.
//! Open `http://localhost:8080/index.html` in browser
//! or [python console client](https://github.com/actix/examples/blob/master/websocket/websocket-client.py)
//! could be used for testing.

#[macro_use]
extern crate diesel;

use std::env;

use actix_files as fs;
use actix_web::web::Data;
use actix_web::{
    get, middleware, post, web, App, HttpResponse, HttpServer, Responder,
};
use awc::http::StatusCode;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use dotenv::dotenv;
use r2d2::Pool;
use crate::models::Room;

// pub mod models;
mod models;
mod schema;
mod websocket;

#[get("/hello")]
async fn hello(_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[cfg(debug_assertions)]
#[get("/rooms")]
async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let connection = pool.get().expect("Could not create connection");
    let res = schema::room::table.load::<Room>(&connection);

    if let Ok(result) = res {
        let tmp = serde_json::to_string(&result).unwrap_or(String::from("Empty vec"));
        HttpResponse::Ok().content_type("application/json").status(StatusCode::OK).body(format!("{:?}", tmp))
    } else {
        HttpResponse::Ok().content_type("application/json").status(StatusCode::INTERNAL_SERVER_ERROR).body("[]")
    }
}

#[cfg(not(debug_assertions))]
#[get("/rooms")]
async fn rooms(pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().content_type("text/plain").status(StatusCode::NOT_FOUND).body("Not available in release mode")
}

#[post("/echo")]
async fn echo(_pool: Data<Pool<ConnectionManager<PgConnection>>>, req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(_pool: Data<Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

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
            .service(hello)
            .service(echo)
            .service(rooms)
            .route("/hey", web::get().to(manual_hello))
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
