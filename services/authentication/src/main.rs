pub mod routes;
pub mod middleware;

use actix_web::{web, App, HttpServer};
use sea_orm::{ConnectOptions, Database};
use std::env;
use std::time::Duration;
use env_logger::Env;
use log::info;
use reqwest::Url;
use webauthn_rs::WebauthnBuilder;
use migration::{Migrator, MigratorTrait};
use storage::s3_manager::S3StorageManager;

pub struct ProviderConfiguration {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_callback_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub jwt_secret: String,
    pub domain_root: String,
    pub origin_secret: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set").trim().to_owned();

    let access_key = env::var("ACCESS_KEY").expect("ACCESS_KEY must be set").trim().to_owned();
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set").trim().to_owned();
    let bucket = env::var("BUCKET").expect("BUCKET must be set").trim().to_owned();
    let endpoint = env::var("ENDPOINT").expect("ENDPOINT must be set").trim().to_owned();

    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set").trim().to_owned();
    let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set").trim().to_owned();
    let google_callback_url = env::var("GOOGLE_CALLBACK_URL").expect("GOOGLE_CLIENT_SECRET must be set").trim().to_owned();

    let github_client_id = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set").trim().to_owned();
    let github_client_secret = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must be set").trim().to_owned();

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set").trim().to_owned();
    let domain_root = env::var("DOMAIN_ROOT").expect("DOMAIN_ROOT must be set").to_owned();
    let rp_origin = env::var("RP_ORIGIN").expect("RP_ORIGIN must be set").to_owned();
    let rp_id = env::var("RP_ID").expect("RP_ID must be set").to_owned();

    let origin_secret = env::var("ORIGIN_SECRET").expect("ORIGIN_SECRET must be set").trim().to_owned();
    
    let rp_origin = Url::parse(&rp_origin).expect("Invalid RP_ORIGIN URL");

    let builder = WebauthnBuilder::new(&rp_id, &rp_origin).expect("Invalid configuration");
    let builder = builder.rp_name("Ledger");

    let provider_configuration = ProviderConfiguration {
        google_client_id,
        google_client_secret,
        google_callback_url,
        github_client_id,
        github_client_secret,
        jwt_secret,
        domain_root,
        origin_secret
    };

    let s3_manager = S3StorageManager::new_s3(
        access_key,
        secret_key,
        bucket,
        endpoint,
    ).await;

    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .test_before_acquire(false);

    let database_client = Database::connect(opt).await.unwrap();
    Migrator::up(&database_client, None).await.unwrap();

    let s3_data = web::Data::new(s3_manager);
    let db_data = web::Data::new(database_client);
    let provider_data = web::Data::new(provider_configuration);
    let webauth = web::Data::new(builder.build().expect("Failed to build WebAuthn instance"));

    info!("Starting user server on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(s3_data.clone())
            .app_data(db_data.clone())
            .app_data(provider_data.clone())
            .app_data(webauth.clone())
            .configure(routes::routes)
            .configure(routes::user::routes)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

