use actix_web::{web, HttpResponse};
use sea_orm::sea_query::prelude::serde_json::json;
use crate::ProviderConfiguration;

#[actix_web::post("logout")]
pub async fn logout(
    _req: actix_web::HttpRequest,
    provider_config: web::Data<ProviderConfiguration>,
) -> HttpResponse {
    let access_cookie = actix_web::cookie::Cookie::build("session", "")
        .path("/")
        .max_age(actix_web::cookie::time::Duration::minutes(15))
        .secure(true)
        .domain(&provider_config.domain_root)
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::None)
        .finish();

    let refresh_cookie = actix_web::cookie::Cookie::build("refresh_token", "")
        .path("/")
        .secure(true)
        .domain(&provider_config.domain_root)
        .max_age(actix_web::cookie::time::Duration::days(30))
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::None)
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(
            json!({
                "message": "Logout successful",
            })
        )
}