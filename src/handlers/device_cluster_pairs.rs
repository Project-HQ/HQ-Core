use actix_web::{HttpRequest, HttpResponse };
use actix_web::web;

use crate::models::device_cluster_pair::DeviceClusterPairList;
use crate::db_connection::{ PgPool, PgPooledConnection };

async fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub async fn list_device_cluster_pairs(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Ok(HttpResponse::Ok().json(DeviceClusterPairList::list(&pg_pool)))
}

use crate::models::device_cluster_pair::NewDeviceClusterPair;

pub async fn add_device_cluster_pair(new_device_cluster_pair: web::Json<NewDeviceClusterPair>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    new_device_cluster_pair.create(&pg_pool)
              .map(|cluster| HttpResponse::Ok().json(cluster))
              .map_err(|e| {HttpResponse::InternalServerError().json(e.to_string())})
}

use crate::models::device_cluster_pair::DeviceClusterPair;

pub async fn get_device_cluster_pair(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    DeviceClusterPair::find(&id, &pg_pool)
        .map(|cluster| HttpResponse::Ok().json(cluster))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn destroy_device_cluster_pair(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    DeviceClusterPair::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub  async fn update_device_cluster_pair(id: web::Path<i32>, new_device_cluster_pair: web::Json<NewDeviceClusterPair>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    DeviceClusterPair::update(&id, &new_device_cluster_pair, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
