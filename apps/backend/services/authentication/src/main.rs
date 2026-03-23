pub mod routes;
pub mod middleware;

use actix_web::{web, App, HttpServer};
use sea_orm::Database;
use std::env;
use env_logger::Env;
use log::info;
use migration::{Migrator, MigratorTrait};
use storage::s3_manager::S3StorageManager;

pub struct ProviderConfiguration {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_callback_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let account_id = env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID must be set");
    let access_key = env::var("R2_ACCESS_KEY").expect("R2_ACCESS_KEY must be set");
    let secret_key = env::var("R2_SECRET_KEY").expect("R2_SECRET_KEY must be set");
    let bucket = env::var("R2_BUCKET").expect("R2_BUCKET must be set");

    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");
    let google_callback_url = env::var("GOOGLE_CALLBACK_URL").expect("GOOGLE_CLIENT_SECRET must be set");

    let github_client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set");
    let github_client_secret = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let provider_configuration = ProviderConfiguration {
        google_client_id,
        google_client_secret,
        google_callback_url,
        github_client_id,
        github_client_secret,
        jwt_secret,
    };

    let s3_manager = S3StorageManager::new(
        access_key,
        secret_key,
        account_id,
        bucket,
    ).await;
    
    let database_client = Database::connect(&database_url).await.unwrap();
    Migrator::up(&database_client, None).await.unwrap();

    let s3_data = web::Data::new(s3_manager);
    let db_data = web::Data::new(database_client);
    let provider_data = web::Data::new(provider_configuration);

    info!("Starting user server on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(s3_data.clone())
            .app_data(db_data.clone())
            .app_data(provider_data.clone())
            .configure(routes::routes)
            .configure(routes::user::routes)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

