use crate::context::AppContext;
use crate::modules::postgres::postgres_service::PostgresService;
use crate::modules::redis::redis_service::RedisService;
use crate::modules::s3::s3_service::S3Service;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::debug;
use log::warn;
use std::sync::Arc;

mod config;
mod context;
mod middleware;
mod modules;
mod routes;
mod scheduler;
mod types;
mod util;

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

    // Ensure the bucket exists in dev (MinIO) environments
    if let Err(e) = s3_service.ensure_bucket().await {
        warn!("S3 bucket check/create failed: {}", e);
    }

    let postgres_service = Arc::new(
        PostgresService::new(&config.postgres.postgres_uri)
            .await
            .unwrap(),
    );

    let redis_service = Arc::new(
        RedisService::new(&config.redis.redis_url)
            .await
            .expect("Failed to connect to Redis"),
    );

    let context = Arc::new(AppContext::new(
        Arc::clone(&s3_service),
        Arc::clone(&postgres_service),
        Arc::clone(&redis_service),
    ));

    let scheduler_context = Arc::clone(&context);
    tokio::spawn(async move {
        scheduler::configure_scheduler()
            .start(scheduler_context)
            .await;
    });

    debug!("Starting server...");

    let app_data = Data::new(Arc::clone(&context));
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .app_data(MultipartFormConfig::default().total_limit(1000 * 1024 * 1024))
            .configure(|cfg| {
                routes::configure_routes(cfg);
            })
    })
    .bind(("::", 8080))?
    .run()
    .await
}
