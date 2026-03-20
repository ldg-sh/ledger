use std::io::ErrorKind;
use std::sync::Arc;
use actix_web::{web, HttpResponse};
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;

#[actix_web::get("info")]
pub async fn info(
    _req: actix_web::HttpRequest,
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser
) -> HttpResponse {
    let user = context.postgres_service.get_user_information(authenticated_user.id.clone()).await;

    match user {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::InternalServerError().body("Failed to retrieve user information")
            }
        }
    }
}