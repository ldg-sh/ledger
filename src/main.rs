use crate::modules::s3_service::S3Service;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::sync::Arc;
use env_logger::Env;
use log::debug;

mod config;
mod modules;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = config::EnvConfig::from_env();
    config::CONFIG.set(config.clone()).unwrap(); // Should panic and exit 

    let s3_service = Arc::new(S3Service::new(
        &config.bucket.s3_access_key,
        &config.bucket.s3_secret_key,
        &config.bucket.bucket_name
    ).expect("Failed to create S3 service"));

    debug!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::clone(&s3_service)))
            .configure(|cfg| {
                routes::configure_routes(cfg);
            })
    })
        .bind(("::", 8080))?
        .run()
        .await

}