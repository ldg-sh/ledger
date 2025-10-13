use crate::modules::grpc::grpc_service::GrpcService;
use crate::modules::postgres::postgres_service::PostgresService;
use crate::modules::s3::s3_service::S3Service;
use actix_multipart::form::MultipartFormConfig;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use env_logger::Env;
use log::debug;
use log::warn;
use std::sync::Arc;
use std::time::Duration;
use tonic::transport::Endpoint;

mod config;
mod middleware;
mod modules;
mod routes;
mod types;
mod util;

pub mod ledger {
    tonic::include_proto!("auth");
}

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

    let grpc_channel = loop {
        match Endpoint::from_shared(config.grpc.url.clone())
            .expect("bad gRPC URL")
            .connect()
            .await
        {
            Ok(ch) => break ch,
            Err(e) => {
                warn!(
                    "gRPC connection to {} failed: {}. Retrying in 2s...",
                    config.grpc.url, e
                );
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    };

    let grpc_service = Arc::new(
        GrpcService::new(grpc_channel, &config.grpc.auth_key)
            .expect("Failed to create gRPC service"),
    );

    debug!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::clone(&s3_service)))
            .app_data(Data::new(Arc::clone(&postgres_service)))
            .app_data(Data::new(Arc::clone(&grpc_service)))
            .app_data(MultipartFormConfig::default().total_limit(1000 * 1024 * 1024))
            .configure(|cfg| {
                routes::configure_routes(cfg);
            })
    })
    .bind(("::", 8080))?
    .run()
    .await
}
