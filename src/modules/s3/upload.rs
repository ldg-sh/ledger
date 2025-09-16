use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::types::builders::CompletedMultipartUploadBuilder;
use aws_sdk_s3::types::{ChecksumAlgorithm};
use aws_sdk_s3::{operation::upload_part::UploadPartOutput, primitives::ByteStream};
use base64::{prelude::BASE64_STANDARD, Engine};
use tokio::sync::Semaphore;
use anyhow::{Result, Context};
use crate::modules::s3::s3_service::S3Service;
use std::{io::Error, sync::Arc};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};

struct CompletedPart {
    pub(super) part_number: u32,
    pub(super) upload_part_output: UploadPartOutput,
}

pub struct ActiveUpload {
    pub(super) upload_id: String,
    pub(super) file_id: String,
    parts: Vec<CompletedPart>,
    pub(super) semaphore: Arc<Semaphore>,
    current_file_size: u64,
}

impl S3Service {
    pub async fn upload_part(
        &self,
        upload_id: &str,
        file_id: &str,
        chunk_number: u32,
        total_chunks: u32,
        chunk_data: Vec<u8>,
        checksum: String,
        database_connection: &DatabaseConnection
    ) -> Result<(), Error> {
        if chunk_data.is_empty() {
            return Err(Error::other(
                "Chunk data is empty",
            ));
        }

        {
            let uploads = self.active_uploads.read().await;
            if let Some(upload) = uploads.iter().find(|u| u.upload_id == upload_id) {
                if upload.file_id != file_id {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "File name does not match the upload ID",
                    ));
                }
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Upload ID not found",
                ));
            }
        }

        let semaphore = {
            let uploads = self.active_uploads.read().await;
            uploads
                .iter()
                .find(|a| a.upload_id == upload_id)
                .map(|a| a.semaphore.clone())
                .unwrap_or_else(|| Arc::new(Semaphore::new(3)))
        };

        let permit = semaphore.acquire_owned().await.map_err(|_| {
            Error::new(
                std::io::ErrorKind::Other,
                "Failed to acquire semaphore permit",
            )
        })?;

        let encoded_checksum = BASE64_STANDARD.encode(hex::decode(&checksum).map_err(|e| {
            Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid checksum format: {}", e),
            )
        })?);

        let mut attempts = 0;
        let completed_part = loop {
            attempts += 1;

            match self
                .client
                .upload_part()
                .bucket(&self.bucket)
                .key(file_id)
                .upload_id(upload_id)
                .part_number(chunk_number as i32)
                .body(ByteStream::from(chunk_data.clone()))
                .checksum_algorithm(ChecksumAlgorithm::Sha256)
                .checksum_sha256(&encoded_checksum)
                .send()
                .await
            {
                Ok(part) => {
                    break part;
                }
                Err(e) => {
                    if attempts >= 2 {
                        match e.code() {
                            Some(code) if code.contains("BadDigest") => {
                                return Err(Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    format!("Checksum mismatch: {}", e),
                                ));
                            }
                            Some(code) => {
                                return Err(Error::other(
                                    format!("Failed to upload part: {}", code),
                                ));
                            }
                            _ => {
                                log::error!("Failed to upload part: {}", e);
                            }
                        }

                        return Err(Error::other(
                            format!("Failed to upload part after 3 attempts: {}", e),
                        ));
                    }

                    log::warn!("Failed to upload part (attempt {}): {}", attempts, e);
                    continue;
                }
            }
        };

        {
            let mut uploads = self.active_uploads.write().await;
            if let Some(upload) = uploads.iter_mut().find(|u| u.upload_id == upload_id) {
                upload.parts.push(CompletedPart {
                    part_number: chunk_number,
                    upload_part_output: completed_part.clone(),
                });
                upload.current_file_size += chunk_data.len() as u64;
            } else {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    "Upload ID not found when recording part",
                ));
            }
        }

        if self.active_uploads.read().await.iter().any(|upload| {
            upload.upload_id == upload_id && upload.parts.len() == total_chunks as usize
        }) {
            let cloned_uploads = self.active_uploads.read().await;

            let active_upload = cloned_uploads
                .iter()
                .find(|upload| upload.upload_id == upload_id);

            // Normalize ETag by stripping surrounding quotes if present.
            fn normalize_etag(etag: &str) -> String {
                let trimmed = etag.trim();
                if trimmed.len() >= 2 && trimmed.starts_with('"') && trimmed.ends_with('"') {
                    trimmed[1..trimmed.len()-1].to_string()
                } else {
                    trimmed.to_string()
                }
            }

            let parts = match active_upload {
                Some(upload) => {
                    let mut parts: Vec<_> = upload
                        .parts
                        .iter()
                        .map(|part| {
                            aws_sdk_s3::types::CompletedPart::builder()
                                .part_number(part.part_number as i32)
                                .e_tag(
                                    normalize_etag(
                                        part
                                            .upload_part_output
                                            .e_tag()
                                            .unwrap_or_default(),
                                    ),
                                )
                                .build()
                        })
                        .collect();

                    parts.sort_by_key(|p| p.part_number);
                    parts
                }
                None => {
                    log::error!("No active upload found with ID: {}", upload_id);
                    return Err(Error::other(
                        format!("No active upload found with ID: {}", upload_id),
                    ));
                }
            };

            match self
                .client
                .complete_multipart_upload()
                .bucket(&self.bucket)
                .key(file_id)
                .upload_id(upload_id)
                .multipart_upload(
                    CompletedMultipartUploadBuilder::default()
                        .set_parts(Some(parts))
                        .build(),
                )
                .send()
                .await
            {
                Ok(result) => result,
                Err(e) => {
                    log::error!(
                        "Failed to complete multipart upload for file {}: {:?}",
                        file_id,
                        e
                    );
                    return Err(Error::other(
                        format!("Failed to complete multipart upload: {}", e),
                    ));
                }
            };

            use entity::file::Entity as FileEntity;
            use sea_orm::ActiveModelTrait;
            use sea_orm::Set;

            let file = FileEntity::find_by_id(file_id)
                .one(database_connection)
                .await
                .map_err(|e| {
                    Error::other(
                        format!("Database query error: {}", e),
                    )
                })?
                .ok_or_else(|| Error::new(std::io::ErrorKind::NotFound, "File not found in database"))?;

            let mut file_model: entity::file::ActiveModel = file.into_active_model();
            file_model.upload_id = Set(upload_id.to_string());
            file_model.file_size = Set(active_upload.unwrap().current_file_size as i64);
            file_model.upload_completed = Set(true);

            file_model
                .update(database_connection)
                .await
                .map_err(|e| {
                        Error::other(
                            format!("Failed to update file in database: {}", e),
                        )
                    })?;

            drop(cloned_uploads);

            let mut map = self.active_uploads.write().await;
            map.retain(|upload| upload.upload_id != upload_id);

            log::info!("Multipart upload completed for file: {}", file_id);
        } else {
            let amount_of_chunks_uploaded = self
                .active_uploads
                .read()
                .await
                .iter()
                .find(|upload| upload.upload_id == upload_id)
                .map_or(0, |upload| upload.parts.len());

            let upload_percent = (amount_of_chunks_uploaded as f64 / total_chunks as f64) * 100.0;
            let upload_percent = (upload_percent * 100.0).round() / 100.0;

            log::info!(
                "Uploaded chunk {}/{} for file {} ({}% complete)",
                amount_of_chunks_uploaded,
                total_chunks,
                file_id,
                upload_percent
            );
        }

        drop(permit);
        Ok(())
    }

    pub async fn initiate_upload(&self, file_id: &str, content_type: &str) -> Result<String> {
        let initiation = self
            .client
            .create_multipart_upload()
            .bucket(&self.bucket)
            .content_type(content_type)
            .key(file_id)
            .send()
            .await
            .context("Failed to send multipart upload request")?;

        let upload_id = initiation
            .upload_id.context("Upload ID missing in S3 response")?;

        self.active_uploads.write().await.push(
            ActiveUpload {
                upload_id: upload_id.clone(),
                file_id: file_id.to_string(),
                parts: Vec::new(),
                semaphore: Arc::new(Semaphore::new(3)),
                current_file_size: 0,
            }
        );

        Ok(upload_id)
    }
}
