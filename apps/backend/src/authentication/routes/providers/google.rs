use std::sync::Arc;
use crate::authentication::success::login_success;
use crate::config::config;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::authentication::routes::providers::Provider;
use crate::context::AppContext;

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
    context: web::Data<Arc<AppContext>>,
) -> HttpResponse {
    let client = reqwest::Client::new();

    let token_resp = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", &query.code),
            ("client_id", &config().auth.google_client_id),
            ("client_secret", &config().auth.google_client_secret),
            ("redirect_uri", &config().auth.google_callback_url),
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

    let user_uuid = context.postgres_service
        .upsert_oauth_user(
            google_user.email,
            google_user.name.unwrap_or("Unknown".to_string()),
            google_user.id,
            google_user.picture,
            Provider::Google,
        )
        .await
        .expect("Failed to sync Google user to database");

    login_success(user_uuid, context.postgres_service.clone()).await
}