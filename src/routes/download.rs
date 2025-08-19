extern crate sanitize_filename;

use crate::modules::s3::s3_service::S3Service;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::http::header::{ACCEPT_RANGES, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::{web, get, head, HttpResponse, Responder};
use std::sync::Arc;

#[derive(MultipartForm)]
pub struct ChunkDownload {
    #[multipart(rename = "fileName")]
    file_name: Text<String>,
    #[multipart(rename = "rangeStart")]
    range_start: Text<u64>,
    #[multipart(rename = "rangeEnd")]
    range_end: Text<u64>,
}

#[derive(MultipartForm)]
pub struct DownloadMetadata {
    #[multipart(rename = "fileName")]
    file_name: Text<String>,
}

#[get("/metadata")]
pub async fn metadata(
    s3_service: web::Data<Arc<S3Service>>,
    MultipartForm(metadata): MultipartForm<DownloadMetadata>,
) -> impl Responder {
    let metadata = s3_service.get_metadata(&metadata.file_name).await.unwrap();

    web::Json(metadata)
}

#[get("")]
pub async fn download(
    s3_service: web::Data<Arc<S3Service>>,
    MultipartForm(download): MultipartForm<ChunkDownload>,
) -> HttpResponse {
    let object_output = match s3_service.download_part(&download.file_name, *download.range_start, *download.range_end).await {
        Ok(object) => {object}
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };

    let mime_type = object_output.content_type().unwrap_or("application/octet-stream");

    HttpResponse::build(StatusCode::PARTIAL_CONTENT)
        .insert_header((ACCEPT_RANGES, "bytes"))
        .insert_header((CONTENT_TYPE, mime_type))
        .body(object_output.body.collect().await.unwrap().into_bytes())
}
