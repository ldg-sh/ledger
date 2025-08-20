extern crate sanitize_filename;

use crate::modules::s3::s3_service::S3Service;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::text::Text;
use actix_web::{Responder, post, web};
use std::io::Read;
use std::sync::Arc;
use crate::modules::postgres::postgres::PostgresService;
use crate::modules::redis::redis::RedisService;

#[derive(MultipartForm)]
pub struct ChunkUploadForm {
    #[multipart(rename = "uploadId")]
    upload_id: Option<Text<String>>,
    #[multipart(rename = "checksum")]
    checksum: Text<String>,
    #[multipart(rename = "fileName")]
    file_name: Text<String>,
    #[multipart(rename = "chunkNumber")]
    chunk_number: Text<u32>,
    #[multipart(rename = "totalChunks")]
    total_chunks: Text<u32>,
    #[multipart(rename = "contentType")]
    content_type: Text<String>,
    #[multipart(rename = "chunk")]
    pub(crate) chunk_data: Vec<actix_multipart::form::tempfile::TempFile>,
}

#[post("")]
pub async fn upload(
    s3_service: web::Data<Arc<S3Service>>,
    MultipartForm(form): MultipartForm<ChunkUploadForm>,
) -> impl Responder {
    let file_name = sanitize_filename::sanitize(&form.file_name.0);
    let chunk_size: u64 = form.chunk_data.iter().map(|f| f.size as u64).sum();
    log::debug!("Chunk size: {} bytes", chunk_size);

    let mut chunk_data = Vec::new();
    for mut file in form.chunk_data {
        let mut file_content = Vec::new();
        file.file.read_to_end(&mut file_content).unwrap();
        chunk_data.extend(file_content);
    }

    if form.chunk_number.0 == 1 {
        log::debug!("Starting new upload for file: {}", file_name);
        let id = s3_service.initiate_upload(file_name.clone().as_str(), form.content_type.as_str()).await;

        let id = match id {
            Ok(id) => id,
            Err(e) => {
                log::error!("Failed to initiate upload for file {}: {}", file_name, e);
                return format!("Failed to initiate upload: {}", e);
            }
        };

        match s3_service
            .upload_part(
                &id,
                &file_name,
                form.chunk_number.0,
                form.total_chunks.0,
                chunk_data.clone(),
                form.checksum.0.clone(),
            )
            .await
        {
            Ok(_) => {
                log::debug!("Successfully uploaded first chunk for file {}", file_name);
            }
            Err(e) => {
                return format!("Failed to upload chunk: {}", e);
            }
        }

        return id;
    } else {
        if form.upload_id.is_none() {
            return "Missing upload ID for chunk upload".to_string();
        }

        let upload_id = form.upload_id.as_ref().unwrap().0.clone();

        let result = s3_service
            .upload_part(
                &upload_id,
                &file_name,
                form.chunk_number.0,
                form.total_chunks.0,
                chunk_data,
                form.checksum.0.clone(),
            )
            .await;

        if let Err(e) = result {
            log::error!(
                "Failed to upload chunk {} of {} for file {}: {}",
                form.chunk_number.0,
                form.total_chunks.0,
                file_name,
                e
            );
            return format!("Failed to upload chunk {}: {}", form.chunk_number.0, e);
        } else {
            log::debug!(
                "Successfully uploaded chunk {} of {} for file {}",
                form.chunk_number.0,
                form.total_chunks.0,
                file_name
            );
        }
    }

    format!(
        "Uploaded chunk {} of {} for file {}",
        form.chunk_number.0, form.total_chunks.0, file_name
    )
}

pub struct CreateUploadForm {
    name: String,
    content_type: String,
}

#[post("/create")]
pub async fn create_upload(
    s3_service: web::Data<Arc<S3Service>>,
    redis: web::Data<Arc<RedisService>>,
    db: web::Data<Arc<PostgresService>>,
    MultipartForm(form): MultipartForm<ChunkUploadForm>,
) -> impl Responder {
// TODO: Saniotize the file name and content type. We dont want to allow anything.
""
}
