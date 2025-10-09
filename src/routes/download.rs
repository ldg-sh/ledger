extern crate sanitize_filename;

use crate::modules::postgres::postgres_service::PostgresService;
use crate::modules::s3::download::GetMetadataResponse;
use crate::modules::s3::s3_service::S3Service;
use actix_web::http::StatusCode;
use actix_web::http::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_TYPE};
use actix_web::{HttpResponse, Responder, get, web};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_object::HeadObjectError;
use sea_orm::sqlx::types::chrono;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_util::io::ReaderStream;

#[derive(Serialize, Deserialize)]
pub struct ChunkDownload {
    #[serde(rename = "rangeStart")]
    range_start: u64,
    #[serde(rename = "rangeEnd")]
    range_end: u64,
}

#[get("/metadata")]
pub async fn metadata(
    s3_service: web::Data<Arc<S3Service>>,
    params: web::Path<(String, String)>,
) -> HttpResponse {
    let metadata = match s3_service
        .get_metadata(params.1.as_ref(), params.0.as_ref())
        .await
    {
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
            return HttpResponse::InternalServerError()
                .body("Error whilst trying to obtain content size in metadata.");
        }
    };

    let mime = match metadata.content_type {
        Some(mime) => mime,
        None => {
            return HttpResponse::InternalServerError()
                .body("Error whilst obtaining content type.");
        }
    };

    let formatted_metadata = GetMetadataResponse {
        content_size,
        metadata: metadata.metadata,
        mime,
    };

    HttpResponse::build(StatusCode::OK).json(web::Json(formatted_metadata))
}

#[get("")]
pub async fn download(
    s3_service: web::Data<Arc<S3Service>>,
    params: web::Path<(String, String)>,
    download: web::Query<ChunkDownload>,
) -> HttpResponse {
    let object_output = match s3_service
        .download_part(
            &params.0,
            &params.1,
            download.range_start,
            download.range_end,
        )
        .await
    {
        Ok(object) => object,
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };

    let mime_type = object_output
        .content_type()
        .unwrap_or("application/octet-stream");

    HttpResponse::build(StatusCode::PARTIAL_CONTENT)
        .insert_header((ACCEPT_RANGES, "bytes"))
        .insert_header((CONTENT_TYPE, mime_type))
        .body(object_output.body.collect().await.unwrap().into_bytes())
}

#[get("/view")]
pub async fn download_full(
    s3_service: web::Data<Arc<S3Service>>,
    params: web::Path<(String, String)>,
) -> HttpResponse {
    let object_output = match s3_service.download_file(&params.0, &params.1).await {
        Ok(object) => object,
        Err(e) => {
            return HttpResponse::InternalServerError().json(e.to_string());
        }
    };

    let mime_type = object_output
        .content_type()
        .unwrap_or("application/octet-stream");

    HttpResponse::Ok()
        .insert_header((ACCEPT_RANGES, "bytes"))
        .insert_header((CONTENT_TYPE, mime_type))
        .insert_header((CONTENT_DISPOSITION, "inline"))
        .streaming(ReaderStream::new(object_output.body.into_async_read()))
}

#[derive(serde::Serialize)]
struct AllFilesSummary {
    file_id: String,
    file_name: String,
    file_size: i64,
    file_type: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[get("/list/all")]
pub async fn list_all_downloads(db: web::Data<Arc<PostgresService>>) -> impl Responder {
    let files = db.list_files().await;
    if let Ok(files) = files {
        let cleaned: Vec<_> = files
            .into_iter()
            .map(|v| AllFilesSummary {
                file_id: v.id,
                file_name: v.file_name,
                file_size: v.file_size,
                file_type: v.file_type,
                created_at: v.created_at,
            })
            .collect();
        return HttpResponse::Ok().json(cleaned);
    }
    HttpResponse::Ok().finish()
}
