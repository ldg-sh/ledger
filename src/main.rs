use crate::modules::s3::s3_service::S3Service;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::debug;
use std::sync::Arc;
use crate::modules::postgres::postgres::PostgresService;
use crate::modules::redis::redis::RedisService;

mod config;
mod modules;
mod routes;
mod util;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = config::EnvConfig::from_env();
    config::CONFIG.set(config.clone()).unwrap();

    let s3_service = Arc::new(
        S3Service::new(
            &config.bucket.s3_access_key,
            &config.bucket.s3_secret_key,
            &config.bucket.bucket_name,
        )
        .expect("Failed to create S3 service"),
    );

    let postgres_service = Arc::new(
        PostgresService::new(
            &config.postgres.postgres_uri,
        )
            .await
            .unwrap()
    );

    let redis_service = Arc::new(
        RedisService::new(
            &config.redis.redis_uri,
        )
    );

    debug!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::clone(&s3_service)))
            .app_data(Data::new(Arc::clone(&postgres_service)))
            .app_data(Data::new(Arc::clone(&redis_service)))
            .app_data(MultipartFormConfig::default().total_limit(1000 * 1024 * 1024))
            .configure(|cfg| {
                routes::configure_routes(cfg);
            })
    })
    .bind(("::", 8080))?
    .run()
    .await
}
