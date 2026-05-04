use crate::routes::user::providers::database::ProviderExtension;
use crate::routes::user::providers::success::login_success;
use crate::ProviderConfiguration;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use log::error;
use sea_orm::sea_query::prelude::chrono;
use sea_orm::DatabaseConnection;

#[actix_web::post("refresh")]
pub async fn refresh(
    req: actix_web::HttpRequest,
    provider_configuration: web::Data<ProviderConfiguration>,
    database: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let start_time = Utc::now();
    println!("Start time is {}", start_time.to_rfc3339());
    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No refresh token found"),
    };

    println!("Reached part 1 in {}ms", (Utc::now() - start_time).num_milliseconds());

    let token_record = match database
        .get_refresh_token(refresh_token.trim().to_string())
        .await
    {
        Ok(record) => record,
        Err(_) => {
            return HttpResponse::Unauthorized().body("Invalid or expired session")
        },
    };
    println!("Reached part 2 in {}ms", (Utc::now() - start_time).num_milliseconds());

    if token_record.expires_at < Utc::now() {
        let _ = database.delete_refresh_token(
            token_record.token,
        ).await;

        return HttpResponse::Unauthorized().body("Session expired");
    }

    println!("Reached part 3 in {}ms", (Utc::now() - start_time).num_milliseconds());

    let db_clone = database.get_ref().clone();
    let token_to_delete = token_record.token.clone();
    tokio::spawn(async move {
        if let Err(e) = db_clone.delete_refresh_token(token_to_delete).await {
            error!("Failed to delete old refresh token in background: {:?}", e);
        }
    });

    println!("Reached part 4 in {}ms", (Utc::now() - start_time).num_milliseconds());
    let res = login_success(token_record.user_id, provider_configuration.jwt_secret.clone(), provider_configuration.domain_root.clone(), database.get_ref().clone()).await;
    println!("Reached part 5 in {}ms", (Utc::now() - start_time).num_milliseconds());
    res
}