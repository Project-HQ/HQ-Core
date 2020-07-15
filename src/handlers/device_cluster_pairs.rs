use actix_web::{HttpRequest, HttpResponse };
use actix_web::web;

use crate::models::device_cluster_pair::DeviceClusterPairList;
use crate::models::device_cluster_pair::DeviceClusterPair;

use crate::db_connection::{ PgPool, PgPooledConnection };

async fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub async fn list_cluster_pairs(cluster_id: web::Path<i32>, _req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Ok(HttpResponse::Ok().json(DeviceClusterPairList::list(&cluster_id, &pg_pool)))
}

use crate::models::device_cluster_pair::NewDeviceClusterPair;

pub async fn assign_device_to_cluster(pair: web::Path<(i32, i32)>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    let c_id: Option<i32> = serde::export::Some(pair.as_ref().0) ;
    let d_id: Option<i32> = serde::export::Some(pair.as_ref().1) ;

    let new_device_cluster_pair = NewDeviceClusterPair{
                                    cluster_id: c_id,
                                    device_id: d_id};

    new_device_cluster_pair.create(&pg_pool)
              .map(|cluster| HttpResponse::Ok().json(cluster))
              .map_err(|e| {HttpResponse::InternalServerError().json(e.to_string())})

}


pub async fn remove_device_from_cluster(pair: web::Path<(i32, i32)>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    let c_id: Option<i32> = serde::export::Some(pair.as_ref().0) ;
    let d_id: Option<i32> = serde::export::Some(pair.as_ref().1) ;

    let new_device_cluster_pair = NewDeviceClusterPair{
                                    cluster_id: c_id,
                                    device_id: d_id};


    DeviceClusterPair::delete_by_pair(&new_device_cluster_pair, &pg_pool)
                                    .map(|id| HttpResponse::Ok().json(id))
                                    .map_err(|e| {HttpResponse::InternalServerError().json(e.to_string())})

}

pub async fn get_devices_from_cluster(c_id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;

    DeviceClusterPair::find_device_relationships(&c_id, &pg_pool)
                                    .map(|id| HttpResponse::Ok().json(id))
                                    .map_err(|e| {HttpResponse::InternalServerError().json(e.to_string())})

}
