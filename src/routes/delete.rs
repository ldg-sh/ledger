extern crate sanitize_filename;

use crate::context::AppContext;
use actix_web::{delete, web, HttpResponse};
use std::sync::Arc;
use crate::middleware::authentication::AuthenticatedUser;
use crate::util::file::build_key_from_path;

#[delete("/{path:.*}")]
pub async fn delete(
    context: web::Data<Arc<AppContext>>,
    path: web::Path<String>,
    authenticated_user: AuthenticatedUser
) -> HttpResponse {
    let s3_service = Arc::clone(&context.into_inner().s3_service);
    let path = build_key_from_path(
        &authenticated_user,
        path.as_str()
    );

    match s3_service.delete(path.as_ref()).await {
        Ok(m) => m,
        Err(e) => {
            log::error!("{e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().finish()
}
