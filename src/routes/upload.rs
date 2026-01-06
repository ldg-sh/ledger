extern crate sanitize_filename;

use crate::context::AppContext;
use crate::types::error::AppError;
use crate::types::file::TCreateFile;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::text::Text;
use actix_web::{HttpResponse, Responder, post, web};
use sea_orm::sqlx::types::{chrono::Utc, uuid};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::Read;
use std::sync::Arc;

#[derive(MultipartForm)]
pub struct ChunkUploadForm {
    #[multipart(rename = "uploadId")]
    upload_id: Option<Text<String>>,
    #[multipart(rename = "checksum")]
    checksum: Text<String>,
    #[multipart(rename = "chunkNumber")]
    chunk_number: Text<u32>,
    #[multipart(rename = "totalChunks")]
    total_chunks: Text<u32>,
    #[multipart(rename = "path")]
    path: Option<Text<String>>,
    #[multipart(rename = "chunk")]
    pub(crate) chunk_data: Vec<actix_multipart::form::tempfile::TempFile>,
}

#[post("")]
pub async fn upload(
    context: web::Data<Arc<AppContext>>,
    MultipartForm(form): MultipartForm<ChunkUploadForm>,
    file_id: web::Path<String>,
) -> impl Responder {
    let context = context.into_inner();
    let s3_service = Arc::clone(&context.s3_service);
    let postgres_service = Arc::clone(&context.postgres_service);

    let chunk_size: u64 = form.chunk_data.iter().map(|f| f.size as u64).sum();
    log::debug!("Chunk size: {} bytes", chunk_size);

    if form.upload_id.is_none() {
        return "Missing upload ID for chunk upload".to_string();
    }

    let mut chunk_data = Vec::new();
    for mut file in form.chunk_data {
        let mut file_content = Vec::new();
        file.file.read_to_end(&mut file_content).unwrap();
        chunk_data.extend(file_content);
    }

    let upload_id = form.upload_id.as_ref().unwrap().0.clone();
    let file_id = file_id.into_inner();
    let key = if let Some(path) = &form.path {
        format!(
            "{}/{}",
            sanitize_filename::sanitize(path.0.clone()),
            sanitize_filename::sanitize(file_id.clone())
        )
    } else {
        sanitize_filename::sanitize(file_id.clone())
    };

    let result = s3_service
        .upload_part(
            &upload_id,
            &file_id,
            &key,
            form.chunk_number.0,
            form.total_chunks.0,
            chunk_data,
            form.checksum.0.clone(),
            postgres_service.as_ref(),
        )
        .await;

    if let Err(e) = result {
        log::error!(
            "Failed to upload chunk {} of {} for file {}: {}",
            form.chunk_number.0,
            form.total_chunks.0,
            file_id,
            e
        );
        return format!("Failed to upload chunk {}: {}", form.chunk_number.0, e);
    } else {
        log::debug!(
            "Successfully uploaded chunk {} of {} for file {}",
            form.chunk_number.0,
            form.total_chunks.0,
            file_id
        );
    }

    format!(
        "Uploaded chunk {} of {} for file {}",
        form.chunk_number.0, form.total_chunks.0, file_id
    )
}

#[derive(MultipartForm)]
pub struct CreateUploadForm {
    #[multipart(rename = "fileName")]
    file_name: Text<String>,
    #[multipart(rename = "contentType")]
    content_type: Text<String>,
    #[multipart(rename = "path")]
    path: Option<Text<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadCache {
    pub upload_id: String,
    pub file_id: String,
    pub file_name: String,
}

// TODO: Use APIResult and standard response format.

#[post("")]
pub async fn create_upload(
    context: web::Data<Arc<AppContext>>,
    MultipartForm(form): MultipartForm<CreateUploadForm>,
) -> impl Responder {
    let context = context.into_inner();
    let s3_service = Arc::clone(&context.s3_service);
    let postgres_service = Arc::clone(&context.postgres_service);

    let content_type = form.content_type.0.clone();
    let file_id = uuid::Uuid::new_v4().to_string();

    let key = if let Some(path) = &form.path {
        format!(
            "{}/{}",
            sanitize_filename::sanitize(path.0.clone()),
            sanitize_filename::sanitize(file_id.clone())
        )
    } else {
        sanitize_filename::sanitize(file_id.clone())
    };

    let upload_id = match s3_service
        .initiate_upload(&file_id, &key, &content_type)
        .await
    {
        Ok(upload_id) => upload_id,
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    };

    match postgres_service
        .create_file(TCreateFile {
            id: file_id.clone(),
            path: if let Some(path) = &form.path {
                sanitize_filename::sanitize(path.0.clone())
            } else {
                String::new()
            },
            file_name: form.file_name.0.clone(),
            upload_id: upload_id.clone(),
            file_size: 0,
            created_at: Utc::now(),
            upload_completed: false,
            file_type: content_type.clone(),
        })
        .await
    {
        Ok(_) => {}
        Err(AppError::AlreadyExists) => return HttpResponse::Conflict().finish(),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    HttpResponse::Ok().json(serde_json::json!({
        "upload_id": upload_id,
        "file_id": file_id,
    }))
}
