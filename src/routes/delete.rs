extern crate sanitize_filename;

use crate::context::AppContext;
use actix_web::{delete, web, HttpResponse};
use std::sync::Arc;

#[delete("")]
pub async fn delete(
    context: web::Data<Arc<AppContext>>,
    file_id: web::Path<String>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.into_inner().s3_service);

    match s3_service.delete(file_id.as_ref()).await {
        Ok(m) => m,
        Err(e) => {
            log::error!("{e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().finish()
}
