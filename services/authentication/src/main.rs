pub mod routes;

use actix_web::{web, App, HttpServer};
use sea_orm::Database;
use std::env;
use storage::s3_manager::S3StorageManager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let account_id = env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID must be set");
    let access_key = env::var("R2_ACCESS_KEY").expect("R2_ACCESS_KEY must be set");
    let secret_key = env::var("R2_SECRET_KEY").expect("R2_SECRET_KEY must be set");
    let bucket = env::var("R2_BUCKET").expect("R2_BUCKET must be set");

    let s3_manager = S3StorageManager::new(
        access_key,
        secret_key,
        account_id,
        bucket,
    ).await;
    
    let database_client = Database::connect(&database_url).await.unwrap();

    let s3_data = web::Data::new(s3_manager);
    let db_data = web::Data::new(database_client);

    HttpServer::new(move || {
        App::new()
            .app_data(s3_data.clone())
            .app_data(db_data.clone())
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

