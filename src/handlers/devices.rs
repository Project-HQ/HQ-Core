use actix_web::{HttpRequest, HttpResponse };
use actix_web::web;

use crate::models::device::DeviceList;
use crate::db_connection::{ PgPool, PgPooledConnection };

async fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub async fn list_devices(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Ok(HttpResponse::Ok().json(DeviceList::list(&pg_pool)))
}

use crate::models::device::NewDevice;

pub async fn add_device(new_device: web::Json<NewDevice>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    new_device.create(&pg_pool)
              .map(|device| HttpResponse::Ok().json(device))
              .map_err(|e| {HttpResponse::InternalServerError().json(e.to_string())})
}

use crate::models::device::Device;

pub async fn show(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Device::find(&id, &pg_pool)
        .map(|device| HttpResponse::Ok().json(device))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn destroy(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Device::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub  async fn update(id: web::Path<i32>, new_device: web::Json<NewDevice>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Device::update(&id, &new_device, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
