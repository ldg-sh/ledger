use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::operation::upload_part::UploadPartOutput;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::builders::CompletedMultipartUploadBuilder;
use aws_sdk_s3::types::{ChecksumAlgorithm, ChecksumType};
use aws_sdk_s3::Client;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::io::Error;
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use crate::config::config;

pub struct S3Service {
    bucket: String,
    client: Client,
    active_uploads: Arc<RwLock<Vec<ActiveUpload>>>,
}

struct ActiveUpload {
    upload_id: String,
    parts: Vec<CompletedPart>,
    semaphore: Arc<Semaphore>,
}

struct CompletedPart {
    part_number: u32,
    upload_part_output: UploadPartOutput,
}

impl S3Service {
    pub fn new(access_key: &str, secret_key: &str, bucket: &str) -> Result<S3Service, Error> {
        let config = aws_sdk_s3::config::Builder::new()
            .region(Region::from_static(&config().bucket.s3_region))
            .behavior_version_latest()
            .endpoint_url(&config().bucket.s3_url)
            .credentials_provider(
                Credentials::builder()
                    .provider_name("backblaze")
                    .access_key_id(access_key)
                    .secret_access_key(secret_key)
                    .build()
            ).build();

        let client = Client::from_conf(config);
        Ok(S3Service { bucket: bucket.to_string(), client, active_uploads: Arc::new(RwLock::new(Vec::new())),  })
    }

    pub async fn upload_part(
        &self,
        upload_id: &str,
        file_name: &str,
        chunk_number: u32,
        total_chunks: u32,
        chunk_data: Vec<u8>,
        checksum: String,
    ) -> Result<(), Error> {
        let semaphore = {
            let uploads = self.active_uploads.read().await;
            uploads
                .iter()
                .find(|a| a.upload_id == upload_id)
                .map(|a| a.semaphore.clone())
                .unwrap_or_else(|| Arc::new(Semaphore::new(3)))
        };

        let permit = semaphore.acquire_owned().await
            .map_err(|_| Error::new(std::io::ErrorKind::Other, "Failed to acquire semaphore permit"))?;

        let encoded_checksum = BASE64_STANDARD.encode(hex::decode(&checksum)
            .map_err(|e| Error::new(std::io::ErrorKind::InvalidData, format!("Invalid checksum format: {}", e)))?);

        let mut attempts = 0;
        let completed_part = loop {
            attempts += 1;

            match self.client.upload_part()
                .bucket(&self.bucket)
                .key(file_name)
                .upload_id(upload_id)
                .part_number(chunk_number as i32)
                .body(ByteStream::from(chunk_data.clone()))
                .checksum_algorithm(ChecksumAlgorithm::Sha256)
                .checksum_sha256(&encoded_checksum)
                .send()
                .await {
                    Ok(part) => {
                        break part;
                    }
                    Err(e) => {
                        if attempts >= 2 {
                            match e.code() {
                                Some(code) if code.contains("BadDigest") => {
                                    return Err(Error::new(std::io::ErrorKind::InvalidData, format!("Checksum mismatch: {}", e)));
                                }
                                Some(code) => {
                                    return Err(Error::new(std::io::ErrorKind::Other, format!("Failed to upload part: {}", code)));
                                }
                                _ => {
                                    log::error!("Failed to upload part: {}", e);
                                }
                            }

                            return Err(Error::new(std::io::ErrorKind::Other, format!("Failed to upload part after 3 attempts: {}", e)));
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
            } else {
                uploads.push(ActiveUpload {
                    upload_id: upload_id.to_string(),
                    parts: vec![CompletedPart {
                        part_number: chunk_number,
                        upload_part_output: completed_part.clone(),
                    }],
                    semaphore: Arc::new(Semaphore::new(3)),
                });
            }
        }

        if self.active_uploads.read().await
            .iter()
            .any(|upload| upload.upload_id == upload_id && upload.parts.len() == total_chunks as usize) {

            let cloned_uploads = self.active_uploads.read().await;
            let parts = cloned_uploads
                .iter()
                .find(|upload| upload.upload_id == upload_id);

            let parts = match parts {
                Some(upload) => {
                    let mut parts: Vec<_> = upload.parts.iter()
                        .map(|part| {
                            aws_sdk_s3::types::CompletedPart::builder()
                                .part_number(part.part_number as i32)
                                .e_tag(part.upload_part_output.e_tag().unwrap_or_default())
                                .build()
                        })
                        .collect();

                    parts.sort_by_key(|p| p.part_number);
                    parts
                }
                None => {
                    log::error!("No active upload found with ID: {}", upload_id);
                    return Err(Error::new(std::io::ErrorKind::Other, format!("No active upload found with ID: {}", upload_id)));
                }
            };

            drop(cloned_uploads);

            match self.client.complete_multipart_upload()
                .bucket(&self.bucket)
                .key(file_name)
                .upload_id(upload_id)
                .multipart_upload(
                    CompletedMultipartUploadBuilder::default()
                        .set_parts(Some(parts))
                        .build()
                )
                .send()
                .await {
                Ok(result) => {
                    result
                }
                Err(e) => {
                    log::error!("Failed to complete multipart upload for file {}: {:?}", file_name, e);
                    return Err(Error::new(std::io::ErrorKind::Other, format!("Failed to complete multipart upload: {}", e)));
                }
            };

            let mut map = self.active_uploads.write().await;
            map.retain(|upload| upload.upload_id != upload_id);

            log::info!("Multipart upload completed for file: {}", file_name);
        } else {
            let amount_of_chunks_uploaded = self.active_uploads.read().await
                .iter()
                .find(|upload| upload.upload_id == upload_id)
                .map_or(0, |upload| upload.parts.len());

            let upload_percent = (amount_of_chunks_uploaded as f64 / total_chunks as f64) * 100.0;
            let upload_percent = (upload_percent * 100.0).round() / 100.0;

            log::info!("Uploaded chunk {}/{} for file {} ({}% complete)", amount_of_chunks_uploaded, total_chunks, file_name, upload_percent);
        }

        drop(permit);
        Ok(())
    }

    pub async fn initiate_upload(
        &self,
        file_name: String,
    ) -> Result<String, String> {
        let initiation = self.client.create_multipart_upload()
            .bucket(&self.bucket)
            .checksum_algorithm(ChecksumAlgorithm::Sha256)
            .checksum_type(ChecksumType::Composite)
            .key(&file_name)
            .send()
            .await;

        match initiation {
            Ok(initiation) => Ok(initiation.upload_id.unwrap_or_default()),
            Err(e) => Err(e.to_string()),
        }
    }
}