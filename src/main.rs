pub mod schema;
pub mod db_connection;
pub mod models;
pub mod handlers;

extern crate actix;
extern crate actix_web;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate openssl;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

use actix_web::{App, HttpServer, web};
use db_connection::establish_connection;

fn main() {

    let sys = actix::System::new("HQ_Core");

    HttpServer::new(
    || App::new()
        .data(establish_connection())
        .service(
            web::resource("/devices")
                .route(web::get().to(handlers::devices::list_devices)) // Gets a list of devices
                .route(web::post().to(handlers::devices::add_device)) // Adds a device
        )
        .service(
            web::resource("/devices/{id}")
                .route(web::get().to(handlers::devices::show))
                .route(web::delete().to(handlers::devices::destroy))
                .route(web::patch().to(handlers::devices::update))
        )
        .service(
            web::resource("/").route(web::get().to(handlers::index::welcome_banner))
        )
    )
    .bind("0.0.0.0:8080").unwrap()
    .run();

    println!("Started http server: http://0.0.0.0:8080");
    let _ = sys.run();
}
