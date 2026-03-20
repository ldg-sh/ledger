use std::sync::Arc;
use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use crate::authentication::routes::providers::Provider;
use crate::authentication::success::login_success;
use crate::config::config;
use crate::context::AppContext;
use crate::types::error::AppError;

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
    context: web::Data<Arc<AppContext>>,
) -> HttpResponse {
    let client = reqwest::Client::new();

    let token_resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", &config().auth.github_client_id),
            ("client_secret", &config().auth.github_client_secret),
            ("code", &query.code),
        ])
        .send()
        .await;

    let Ok(resp) = token_resp else {
        return AppError::Internal("GitHub request failed".to_string()).error_response();
    };

    let token: GitHubTokenResponse = match resp.json().await {
        Ok(t) => t,
        Err(e) => return {
            println!("Failed to parse GitHub access token: {}", e);
            AppError::Internal("Failed to parse GitHub response".to_string()).error_response()
        },
    };

    let user_info = client
        .get("https://api.github.com/user")
        .header("User-Agent", "NextJS-Rust-App")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
        .unwrap()
        .json::<GitHubUser>()
        .await
        .unwrap();

    let res = context.postgres_service.upsert_oauth_user(
        user_info.email,
        user_info.login,
        user_info.id.to_string(),
        Some(user_info.avatar_url),
        Provider::GitHub
    ).await;

    login_success(res.unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()), context.postgres_service.clone()).await
}