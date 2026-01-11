extern crate sanitize_filename;

use crate::context::AppContext;
use actix_web::{get, web, HttpResponse};
use std::sync::Arc;

#[get("")]
pub async fn test(context: web::Data<Arc<AppContext>>) -> HttpResponse {
    let s3_service = Arc::clone(&context.into_inner().s3_service);

    let e = match s3_service.list_files(None).await {
        Ok(e) => e,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        },
    };

    HttpResponse::Ok().body(format!("{:?}", e))
}
