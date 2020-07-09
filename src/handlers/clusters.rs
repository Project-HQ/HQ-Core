use actix_web::{HttpRequest, HttpResponse };
use actix_web::web;

use crate::models::cluster::ClusterList;
use crate::db_connection::{ PgPool, PgPooledConnection };

async fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, HttpResponse> {
    pool
    .get()
    .map_err(|e| {
        HttpResponse::InternalServerError().json(e.to_string())
    })
}

pub async fn list_clusters(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Ok(HttpResponse::Ok().json(ClusterList::list(&pg_pool)))
}

use crate::models::cluster::NewCluster;

pub async fn add_cluster(new_cluster: web::Json<NewCluster>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    new_cluster.create(&pg_pool)
              .map(|cluster| HttpResponse::Ok().json(cluster))
              .map_err(|e| {HttpResponse::InternalServerError().json(e.to_string())})
}

use crate::models::cluster::Cluster;

pub async fn get_cluster(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Cluster::find(&id, &pg_pool)
        .map(|cluster| HttpResponse::Ok().json(cluster))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub async fn destroy_cluster(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Cluster::destroy(&id, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}

pub  async fn update_cluster(id: web::Path<i32>, new_cluster: web::Json<NewCluster>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool).await?;
    Cluster::update(&id, &new_cluster, &pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|e| {
            HttpResponse::InternalServerError().json(e.to_string())
        })
}
