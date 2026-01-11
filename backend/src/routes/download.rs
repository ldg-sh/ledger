extern crate sanitize_filename;

use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::util::file::build_key;
use actix_web::http::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_TYPE};
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse};
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

#[get("")]
pub async fn download(
    context: web::Data<Arc<AppContext>>,
    file_id: web::Path<String>,
    download: web::Query<ChunkDownload>,
    authenticated_user: AuthenticatedUser
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let file = postgres_service.get_file(
        &file_id.clone(),
        &authenticated_user.id
    ).await;

    let key = match file {
        Ok(Some(f)) => {
            build_key(&authenticated_user, f.id.as_ref())
        },
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to retrieve file metadata."),
    };

    let object_output = match s3_service
        .download_part(&key, download.range_start, download.range_end)
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
    context: web::Data<Arc<AppContext>>,
    file_id: web::Path<String>,
    authenticated_user: AuthenticatedUser
) -> HttpResponse {
    let postgres_service = Arc::clone(&context.clone().into_inner().postgres_service);
    let s3_service = Arc::clone(&context.into_inner().s3_service);

    let file = postgres_service.get_file(
        &file_id.clone(),
        &authenticated_user.id
    ).await;

    if file.is_err() {
        return HttpResponse::InternalServerError().body(
            "Failed to retrieve file metadata."
        )
    }

    if file.as_ref().unwrap().is_none() {
        return HttpResponse::NotFound().finish();
    }

    let path = build_key(&authenticated_user, &file_id);

    let object_output = match s3_service.download_file(&path).await {
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
