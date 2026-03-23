use crate::routes::user::providers::database::ProviderExtension;
use crate::routes::user::providers::success::login_success;
use crate::routes::user::providers::Provider;
use crate::ProviderConfiguration;
use actix_web::{web, HttpResponse};
use log::error;
use sea_orm::DatabaseConnection;
use sea_orm::sea_query::prelude::serde_json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
}

#[derive(Deserialize)]
pub struct GitHubTokenResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Deserialize)]
pub struct GitHubUser {
    pub id: i64,
    pub avatar_url: String,
    pub email: String,
    pub login: String,
}

#[actix_web::get("github")]
pub async fn github_callback(
    query: web::Query<AuthRequest>,
    provider_config: web::Data<ProviderConfiguration>,
    database: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let client = reqwest::Client::new();

    let token_resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", provider_config.github_client_id.clone()),
            ("client_secret", provider_config.github_client_secret.clone()),
            ("code", query.code.clone()),
        ])
        .send()
        .await;

    let resp = match token_resp {
        Ok(r) => r,
        Err(e) => {
            error!("Network/Request error: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let body_text = resp.text().await.unwrap_or_else(|_| "Empty body".to_string());

    let token: GitHubTokenResponse = match serde_json::from_str(&body_text) {
        Ok(t) => t,
        Err(e) => {
            error!("JSON Parse Error: {:?}. Raw body was: {}", e, body_text);
            return HttpResponse::BadRequest().body(format!("Failed to parse GitHub response: {}", body_text));
        }
    };

    let user_info = client
        .get("https://api.github.com/user")
        .header("User-Agent", "Ledger")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .unwrap()
        .json::<GitHubUser>()
        .await
        .unwrap();

    let res = database.upsert_oauth_user(
        user_info.email,
        user_info.login,
        user_info.id.to_string(),
        Some(user_info.avatar_url),
        Provider::GitHub
    ).await;

    login_success(res.unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()), provider_config.jwt_secret.clone(), database.get_ref().clone()).await
}