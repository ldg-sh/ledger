use crate::routes::user::providers::database::ProviderExtension;
use crate::routes::user::providers::success::login_success;
use crate::ProviderConfiguration;
use actix_web::{web, HttpResponse};
use sea_orm::sea_query::prelude::chrono;
use sea_orm::DatabaseConnection;

#[actix_web::post("refresh")]
pub async fn refresh(
    req: actix_web::HttpRequest,
    provider_configuration: web::Data<ProviderConfiguration>,
    database: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No refresh token found"),
    };

    println!("Received refresh token: {}", refresh_token);

    let token_record = match database
        .get_refresh_token(refresh_token.trim().to_string())
        .await
    {
        Ok(record) => record,
        Err(error) => {
            println!("Invalid refresh token: {}", error);
            return HttpResponse::Unauthorized().body("Invalid or expired session")
        },
    };

    if token_record.expires_at < chrono::Utc::now() {
        let _ = database.delete_refresh_token(
            token_record.token,
        ).await;

        return HttpResponse::Unauthorized().body("Session expired");
    }

    let _ = database.delete_refresh_token(token_record.token).await;
    login_success(token_record.user_id, provider_configuration.jwt_secret.clone(), provider_configuration.domain_root.clone(), database.get_ref().clone()).await
}