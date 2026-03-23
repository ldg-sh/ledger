use actix_web::HttpResponse;
use sea_orm::sea_query::prelude::serde_json::json;

#[actix_web::post("logout")]
pub async fn logout(
    _req: actix_web::HttpRequest,
) -> HttpResponse {
    let access_cookie = actix_web::cookie::Cookie::build("session", "")
        .path("/")
        .max_age(actix_web::cookie::time::Duration::minutes(15))
        .secure(true)
        .finish();

    let refresh_cookie = actix_web::cookie::Cookie::build("refresh_token", "")
        .path("/auth/refresh")
        .secure(true)
        .max_age(actix_web::cookie::time::Duration::days(30))
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