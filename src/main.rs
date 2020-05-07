use actix_web::{
    middleware, web, App, HttpResponse, HttpServer,
};

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
struct Device {
    name: String,
    id: i32,
    description: String,
    owner: String,
    date_added: DateTime<Utc>,
    date_last_received: DateTime<Utc>,
    last_message: String,
    storage_type: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    device_id: i32,
    date_received: DateTime<Utc>,
    last_message: String,
    int_data: i32,
    str_data: String,
    float_data: f32,
    is_file: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct Cluster {
    id: i32,
    date_added: DateTime<Utc>,
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ClusterMap {
    group_id: i32,
    device_id: i32
}

async fn add_device(item: web::Json<Device>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

async fn delete_device(item: web::Json<Device>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

async fn set_device(item: web::Json<Device>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

async fn log_data(item: web::Json<Data>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/devices").route(web::post().to(add_device))) // add device
            .service(web::resource("/devices").route(web::put().to(set_device))) // set device
            .service(web::resource("/devices").route(web::delete().to(delete_device))) // delete device
            .service(web::resource("/logs").route(web::post().to(log_data))) // log data
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

