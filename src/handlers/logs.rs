use actix_web::{HttpRequest, HttpResponse };
use actix_web::web;

use crate::models::log::LogList;
use crate::db_connection::{ PgPool, PgPooledConnection };

async fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub async fn list_logs(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Ok(HttpResponse::Ok().json(LogList::list(&pg_pool)))
}

use crate::models::log::NewLog;

pub async fn add_log(new_log: web::Json<NewLog>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    new_log.create(&pg_pool)
              .map(|log| HttpResponse::Ok().json(log))
              .map_err(|e| {HttpResponse::InternalServerError().json(e.to_string())})
}

use crate::models::log::Log;

pub async fn get_log(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Log::find(&id, &pg_pool)
        .map(|log| HttpResponse::Ok().json(log))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn get_device_logs(device_id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;

    Log::get_logs_by_device(&device_id, &pg_pool)
        .map(|log| HttpResponse::Ok().json(log))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn destroy_log(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Log::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub  async fn update_log(id: web::Path<i32>, new_log: web::Json<NewLog>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Log::update(&id, &new_log, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
