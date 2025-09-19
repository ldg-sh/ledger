extern crate sanitize_filename;

use crate::modules::s3::s3_service::S3Service;
use actix_web::{HttpResponse, delete, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct DeleteRequest {
    #[serde(rename = "fileId")]
    file_id: String,
}

#[delete("/delete")]
pub async fn delete(
    s3_service: web::Data<Arc<S3Service>>,
    metadata: web::Query<DeleteRequest>,
) -> HttpResponse {
    match s3_service.delete(&metadata.file_id).await {
        Ok(m) => m,
        Err(e) => {
            log::error!("{e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().finish()
}
