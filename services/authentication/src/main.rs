use std::env;
use actix_web::{web, App, HttpServer};
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use sea_orm::Database;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let account_id = env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID must be set");
    let access_key = env::var("R2_ACCESS_KEY").expect("R2_ACCESS_KEY must be set");
    let secret_key = env::var("R2_SECRET_KEY").expect("R2_SECRET_KEY must be set");

    let endpoint_url = format!("https://{}.r2.cloudflarestorage.com", account_id);

    let config = aws_config::defaults(BehaviorVersion::latest())
        .credentials_provider(Credentials::new(access_key, secret_key, None, None, "R2"))
        .region(Region::new("auto"))
        .endpoint_url(endpoint_url)
        .load()
        .await;

    let s3_client = Client::new(&config);
    let database_client = Database::connect(&database_url).await.unwrap();

    let s3_data = web::Data::new(s3_client);
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

