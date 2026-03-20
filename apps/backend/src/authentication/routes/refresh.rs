use std::sync::Arc;
use actix_web::{web, HttpResponse};
use crate::context::AppContext;
use crate::util::auth::generate_access_token;

#[actix_web::post("refresh")]
pub async fn refresh_access_token(
    req: actix_web::HttpRequest,
    context: web::Data<Arc<AppContext>>,
) -> HttpResponse {
    let refresh_token = match req.cookie("refresh_token") {
        Some(c) => c.value().to_string(),
        None => return HttpResponse::Unauthorized().body("No refresh token found"),
    };

    let token_record = match context.postgres_service
        .get_refresh_token(refresh_token)
        .await
    {
        Ok(record) => record,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired session"),
    };

    if token_record.expires_at < chrono::Utc::now() {
        let _ = context.postgres_service.delete_refresh_token(
            token_record.token,
        ).await;
        
        return HttpResponse::Unauthorized().body("Session expired");
    }

    let new_access_token = generate_access_token(token_record.user_id);

    HttpResponse::Ok().json(serde_json::json!({
        "access_token": new_access_token
    }))
}