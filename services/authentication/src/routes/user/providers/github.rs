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
    pub email: Option<String>,
    pub login: String,
}

#[derive(Deserialize)]
pub struct GitHubEmail {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
    pub visibility: Option<String>,
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
    let clean_body = body_text.trim_start_matches('\u{feff}').trim();

    let token: GitHubTokenResponse = match serde_json::from_str(&clean_body) {
        Ok(t) => t,
        Err(e) => {
            error!("JSON Parse Error: {:?}. Raw body was: {}", e, clean_body);
            return HttpResponse::BadRequest().body(format!("Failed to parse GitHub response: {}", clean_body));
        }
    };

    let user_info = match client
        .get("https://api.github.com/user")
        .header("User-Agent", "Ledger")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
    {
        Ok(response) => {
            match response.text().await {
                Ok(body_text) => {
                    match serde_json::from_str::<GitHubUser>(&body_text) {
                        Ok(user) => user,
                        Err(e) => {
                            error!("Failed to parse GitHub user JSON: {:?}", e);
                            error!("Raw GitHub user body was: {}", body_text);

                            return HttpResponse::InternalServerError()
                                .body("Failed to parse GitHub response".to_string());
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read GitHub response body text: {:?}", e);
                    return HttpResponse::InternalServerError().body("Failed to read upstream response");
                }
            }
        }
        Err(e) => {
            error!("Network request to GitHub failed: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to retrieve user information from GitHub");
        }
    };

    let user_email: Option<String> = match client
        .get("https://api.github.com/user/emails")
        .header("User-Agent", "Ledger")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(body_text) => match serde_json::from_str::<Vec<GitHubEmail>>(&body_text) {
                Ok(emails) => emails
                    .into_iter()
                    .find(|e| e.primary && e.verified)
                    .map(|e| e.email),
                Err(e) => {
                    error!("Failed to parse GitHub emails JSON: {:?}. Raw body: {}", e, body_text);
                    None
                }
            },
            Err(e) => {
                error!("Failed to read GitHub emails response body: {:?}", e);
                None
            }
        },
        Err(e) => {
            error!("Network request to GitHub /user/emails failed: {:?}", e);
            None
        }
    };

    if user_email.is_none() {
        return HttpResponse::InternalServerError().body("Failed to retrieve user email");
    }

    let res = database.upsert_oauth_user(
        user_email.unwrap(),
        user_info.login,
        user_info.id.to_string(),
        Some(user_info.avatar_url),
        Provider::GitHub
    ).await;

    login_success(res.unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()), provider_config.jwt_secret.clone(), provider_config.domain_root.clone(), database.get_ref().clone()).await
}