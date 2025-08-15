extern crate sanitize_filename;

use std::io::Read;
use std::sync::Arc;
use actix_multipart::form::MultipartForm;
use actix_multipart::form::text::Text;
use actix_web::{post, web, Responder};
use crate::r2_service::R2Service;

#[derive(MultipartForm)]
pub struct ChunkUploadForm {
    #[multipart(rename = "uploadId")]
    upload_id: Option<Text<String>>,
    #[multipart(rename = "fileName")]
    file_name: Text<String>,
    #[multipart(rename = "chunkNumber")]
    chunk_number: Text<u32>,
    #[multipart(rename = "totalChunks")]
    total_chunks: Text<u32>,
    #[multipart(rename = "chunk")]
    pub(crate) chunk_data: Vec<actix_multipart::form::tempfile::TempFile>,
}

#[post("/upload")]
pub async fn upload(r2_service: web::Data<Arc<R2Service>>, MultipartForm(form): MultipartForm<ChunkUploadForm>) -> impl Responder {
    let file_name = sanitize_filename::sanitize(&form.file_name.0);
    let chunk_size: u64 = form.chunk_data.iter().map(|f| f.size as u64).sum();
    log::info!("Chunk size: {} bytes", chunk_size);

    let mut chunk_data = Vec::new();
    for mut file in form.chunk_data {
        let mut file_content = Vec::new();
        file.file.read_to_end(&mut file_content).unwrap();
        chunk_data.extend(file_content);
    }

    if form.chunk_number.0 == 1 {
        log::info!("Starting new upload for file: {}", file_name);
        let id = r2_service.initiate_upload(file_name.clone()).await.expect("Failed to initiate upload");
        log::info!("Initiated upload for file: {} with upload ID: {}", file_name, id);
        r2_service.upload_r2(
            &id,
            &file_name,
            form.chunk_number.0,
            form.total_chunks.0,
            chunk_data.clone(),
        ).await.expect("Failed to upload first chunk");

        return id;
    } else {
        log::info!("Continuing upload for file: {}", file_name);
        if form.upload_id.is_none() {
            return "Missing upload ID for chunk upload".to_string();
        }

        let upload_id = form.upload_id.as_ref().unwrap().0.clone();
        let result = r2_service.upload_r2(
            &upload_id,
            &file_name,
            form.chunk_number.0,
            form.total_chunks.0,
            chunk_data,
        ).await;

        if let Err(e) = result {
            log::error!("Failed to upload chunk {} of {} for file {}: {}", form.chunk_number.0, form.total_chunks.0, file_name, e);
            return format!("Failed to upload chunk {}: {}", form.chunk_number.0, e);
        } else {
            log::info!("Successfully uploaded chunk {} of {} for file {}", form.chunk_number.0, form.total_chunks.0, file_name);
        }
    }

    format!("Uploaded chunk {} of {} for file {}", form.chunk_number.0, form.total_chunks.0, file_name)
}
