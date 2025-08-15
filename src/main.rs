mod upload;
mod r2_service;

use crate::r2_service::R2Service;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::sync::Arc;
use env_logger::Env;
use actix_web::middleware::Logger;
use log::debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::dotenv().ok();

    let access_token = std::env::var("R2_ACCESS_KEY").expect("R2_ACCESS_TOKEN not set");
    let secret_key = std::env::var("R2_SECRET_KEY").expect("R2_SECRET_KEY not set");

    let r2_service = Arc::new(R2Service::new(
        &access_token,
        &secret_key,
    ).expect("Failed to create R2 service"));

    debug!("Starting server...");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::clone(&r2_service)))
            .service(upload::upload)
    })
        .bind(("::", 3000))?
        .run()
        .await

}