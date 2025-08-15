mod upload;
mod r2_service;

use crate::r2_service::R2Service;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let account_id = std::env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID not set");
    let access_token = std::env::var("R2_ACCESS_TOKEN").expect("R2_ACCESS_TOKEN not set");
    let secret_key = std::env::var("R2_SECRET_KEY").expect("R2_SECRET_KEY not set");

    let r2_service = Arc::new(R2Service::new(
        &account_id,
        &access_token,
        &secret_key,
    ).expect("Failed to create R2 service"));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Arc::clone(&r2_service)))
            .service(upload::upload)
    })
        .bind(("::", 3000))?
        .run()
        .await

}