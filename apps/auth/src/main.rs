use crate::db::postgres_service::PostgresService;
use crate::grpc::authentication;
use crate::routes::configure_routes;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::{error, info};

pub mod config;
pub mod db;
pub mod grpc;
pub mod routes;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = config::EnvConfig::from_env();
    config::CONFIG.set(config.clone()).unwrap();

    info!("Starting postgres...");
    let postgres_service = Arc::new(
        PostgresService::new(&config.db_url)
            .await
            .expect("Failed to initialize PostgresService"),
    );
    info!("Started postgres!");

    let grpc_addr = format!("0.0.0.0:{}", config.grpc.port).parse()?;
    let grpc_service = authentication::server(postgres_service.clone());

    let http_addr = format!("0.0.0.0:{}", config.port);
    let postgres_clone = postgres_service.clone();

    let http_server = tokio::spawn(async move {
        info!("Starting HTTP server on {}", http_addr);
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(Arc::clone(&postgres_clone)))
                .configure(configure_routes)
        })
        .bind(http_addr)
        .expect("Failed to bind HTTP server")
        .run()
        .await
        .expect("HTTP server crashed");
    });

    info!("Starting gRPC server on {}", grpc_addr);
    let grpc_server = Server::builder().add_service(grpc_service).serve(grpc_addr);

    tokio::select! {
        _ = grpc_server => {
            error!("gRPC server exited!");
        }
        _ = http_server => {
            error!("HTTP server exited!");
        }
    }

    Ok(())
}
