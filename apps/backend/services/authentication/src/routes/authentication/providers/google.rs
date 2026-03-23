use crate::routes::authentication::providers::database::ProviderExtension;
use crate::routes::authentication::providers::success::login_success;
use crate::routes::authentication::providers::Provider;
use crate::ProviderConfiguration;
use actix_web::{web, HttpResponse};
use sea_orm::sea_query::prelude::serde_json;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub picture: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
}
#[actix_web::get("google")]
pub async fn google_callback(
    query: web::Query<AuthRequest>,
    provider_config: web::Data<ProviderConfiguration>,
    database: web::Data<DatabaseConnection>
) -> HttpResponse {
    let client = reqwest::Client::new();

    let token_resp = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", &query.code),
            ("client_id", &provider_config.google_client_id),
            ("client_secret", &provider_config.google_client_secret),
            ("redirect_uri", &provider_config.google_callback_url),
            ("grant_type", &"authorization_code".to_string()),
        ])
        .send()
        .await
        .unwrap();

    let token_body = token_resp.text().await.unwrap();

    let token_data: serde_json::Value = serde_json::from_str(&token_body).unwrap();
    let access_token = token_data["access_token"].as_str().expect("Access token missing in response");

    let user_resp = client
        .get("https://www.googleapis.com/oauth2/v1/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap();

    let user_body = user_resp.text().await.unwrap();

    let google_user: GoogleUser = serde_json::from_str(&user_body).unwrap();

    let user_uuid = database
        .upsert_oauth_user(
            google_user.email,
            google_user.name.unwrap_or("Unknown".to_string()),
            google_user.id,
            google_user.picture,
            Provider::Google,
        )
        .await
        .expect("Failed to sync Google user to database");

    login_success(user_uuid, provider_config.jwt_secret.clone(), database.as_ref().clone()).await
}