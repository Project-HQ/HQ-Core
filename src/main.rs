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
extern crate chrono;

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
            web::resource("/device/{id}")
                .route(web::get().to(handlers::devices::show))
                .route(web::delete().to(handlers::devices::destroy))
                .route(web::patch().to(handlers::devices::update))
        )
        .service(
            web::resource("/").route(web::get().to(handlers::index::welcome_banner))
        )
        .service(
            web::resource("/logs")
                .route(web::get().to(handlers::logs::list_logs)) // Gets a list of logs
                .route(web::post().to(handlers::logs::add_log)) // Adds a log
        )
        .service(
            web::resource("/log/{id}")
                .route(web::get().to(handlers::logs::get_log))
                .route(web::delete().to(handlers::logs::destroy_log))
                .route(web::patch().to(handlers::logs::update_log))
        )
        .service(
            web::resource("/clusters")
                .route(web::get().to(handlers::clusters::list_clusters)) // Gets a list of clusters
                .route(web::post().to(handlers::clusters::add_cluster)) // Adds a cluster
        )
        .service(
            web::resource("/cluster/{id}")
                .route(web::get().to(handlers::clusters::get_cluster))
                .route(web::delete().to(handlers::clusters::destroy_cluster))
                .route(web::patch().to(handlers::clusters::update_cluster))
        )

        .service(   
            web::resource("/cluster/{cluster_id}/assign/{device_id}") 
            .route(web::get().to(handlers::device_cluster_pairs::assign_device_to_cluster)) // Define a cluster relationship
        )
        .service(   
            web::resource("/cluster/{cluster_id}/remove/{device_id}") 
            .route(web::get().to(handlers::device_cluster_pairs::remove_device_from_cluster)) // Define a cluster relationship
        )

    )
    .bind("0.0.0.0:8080").unwrap()
    .run();

    println!("Started http server: http://0.0.0.0:8080");
    let _ = sys.run();
}
