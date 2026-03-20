use actix_web::HttpResponse;
use uuid::Uuid;
use crate::util::auth::generate_access_token;

pub async fn login_success(user_id: String) -> HttpResponse {
    let raw_refresh_token = Uuid::new_v4().to_string();

    let access_cookie = actix_web::cookie::Cookie::build("session", generate_access_token(user_id))
        .path("/")
        .max_age(actix_web::cookie::time::Duration::minutes(15))
        .secure(true)
        .finish();

    let refresh_cookie = actix_web::cookie::Cookie::build("refresh_token", raw_refresh_token)
        .path("/auth/refresh")
        .secure(true)
        .max_age(actix_web::cookie::time::Duration::days(30))
        .finish();

    HttpResponse::Ok()
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .body("Login successful")
}