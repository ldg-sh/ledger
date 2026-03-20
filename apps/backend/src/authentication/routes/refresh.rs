use std::sync::Arc;
use actix_web::{web, HttpResponse};
use crate::authentication::success::login_success;
use crate::context::AppContext;

#[actix_web::post("refresh")]
pub async fn refresh(
    req: actix_web::HttpRequest,
    context: web::Data<Arc<AppContext>>,
) -> HttpResponse {
    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No refresh token found"),
    };

    println!("Received refresh token: {}", refresh_token);

    let token_record = match context.postgres_service
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
        let _ = context.postgres_service.delete_refresh_token(
            token_record.token,
        ).await;

        println!("Refresh token expired");

        return HttpResponse::Unauthorized().body("Session expired");
    }

    login_success(token_record.user_id, context.postgres_service.clone()).await
}