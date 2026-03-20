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
    println!("{:?}", authenticated_user);
    let user = context.postgres_service.get_user_information(authenticated_user.id.clone()).await;

    if user.is_err() {
        println!("Error retrieving user information: {:?}", user.err());
        return HttpResponse::InternalServerError().body("Failed to retrieve user information");
    }

    HttpResponse::Ok().json(user.unwrap())
}