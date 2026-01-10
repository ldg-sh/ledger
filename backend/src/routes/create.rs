use std::sync::Arc;
use actix_web::{post, web, HttpResponse};
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::util::file::build_key_from_path;

#[post("/directory/{path:.*}")]
pub async fn create_directory(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.into_inner().s3_service);
    let key = build_key_from_path(
        &authenticated_user,
        &path.into_inner(),
    );

    let dir_creation = s3_service.create_blank_directory(
        &key,
    ).await;

    if dir_creation.is_err() {
        log::error!("Directory creation failed: {:?}", dir_creation.unwrap_err());
        return HttpResponse::InternalServerError().body(
            "Failed to create directory."
        );
    }

    println!("Created directory with key: {}", key);

    HttpResponse::Ok().finish()
}