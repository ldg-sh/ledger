use crate::routes::user::providers::database::ProviderExtension;
use actix_web::HttpResponse;
use chrono::Duration;
use common::util::authentication::generate_access_token;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::prelude::chrono;
use sea_orm::sea_query::prelude::serde_json::json;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn login_success(user_id: String, jwt_secret: String, database: DatabaseConnection) -> HttpResponse {
    let raw_refresh_token = Uuid::new_v4().to_string();

    let access_cookie = actix_web::cookie::Cookie::build("session", generate_access_token(&user_id, &jwt_secret))
        .path("/")
        .max_age(actix_web::cookie::time::Duration::minutes(15))
        .secure(true)
        .finish();

    let refresh_cookie = actix_web::cookie::Cookie::build("refresh_token", raw_refresh_token.clone())
        .path("/auth/refresh")
        .secure(true)
        .max_age(actix_web::cookie::time::Duration::days(30))
        .finish();
    
    let _ = database.store_refresh_token(
        user_id.clone(),
        raw_refresh_token,
        DateTimeWithTimeZone::from(chrono::Utc::now() + Duration::days(30)),
    ).await;

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .json(
            json!({
                "message": "Login successful",
                "user_id": user_id
            })
        )
}