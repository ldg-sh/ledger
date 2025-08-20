extern crate sanitize_filename;

use crate::modules::s3::download::GetMetadataResponse;
use crate::modules::s3::s3_service::S3Service;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::http::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::{web, get, HttpResponse};
use tokio_util::io::ReaderStream;
use std::sync::Arc;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_object::HeadObjectError;

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
) -> HttpResponse {
    let metadata = match s3_service.get_metadata(&metadata.file_name).await {
        Ok(m) => m,
        Err(SdkError::ServiceError(se)) if matches!(se.err(), HeadObjectError::NotFound(_)) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            log::error!("{e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    let content_size = match metadata.content_length {
        Some(size) => size,
        None => {
            return HttpResponse::InternalServerError().body("Error whilst trying to obtain content size in metadata.")
        },
    };

    let mime = match metadata.content_type {
        Some(mime) => mime,
        None => {
            return HttpResponse::InternalServerError().body("Error whilst obtaining content type.")
        },
    };

    let formatted_metadata = GetMetadataResponse {
        content_size: content_size,
        metadata: metadata.metadata,
        mime: mime
    };

    HttpResponse::build(StatusCode::OK)
        .insert_header((ACCEPT_RANGES, "bytes"))
        .json(web::Json(formatted_metadata))
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

#[get("/view/{key}")]
pub async fn download_full(
    s3_service: web::Data<Arc<S3Service>>,
    key: web::Path<String>
) -> HttpResponse {
    let object_output = match s3_service.download_file(&key).await {
        Ok(object) => {object}
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };

    let mime_type = object_output.content_type().unwrap_or("application/octet-stream");

    HttpResponse::Ok()
        .insert_header((ACCEPT_RANGES, "bytes"))
        .insert_header((CONTENT_TYPE, mime_type))
        .insert_header((CONTENT_DISPOSITION, "inline"))
        .streaming(ReaderStream::new(object_output.body.into_async_read()))
}
