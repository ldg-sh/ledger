use crate::modules::r2_service::R2Service;
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

    let r2_service = Arc::new(R2Service::new(
        &config.bucket.r2_access_key,
        &config.bucket.r2_secret_key,
        &config.bucket.bucket_name
    ).expect("Failed to create R2 service"));

    debug!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::clone(&r2_service)))
            .configure(|cfg| {
                routes::configure_routes(cfg);
            })
    })
        .bind(("::", 8080))?
        .run()
        .await

}