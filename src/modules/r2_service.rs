use std::ops::Deref;
use std::sync::{Arc};
use s3::{Bucket, Region};
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::serde_types::Part;
use tokio::sync::{RwLock, Semaphore};
use crate::config::config;

pub struct R2Service {
    bucket: Bucket,
    active_uploads: Arc<RwLock<Vec<ActiveUpload>>>,
}

struct ActiveUpload {
    upload_id: String,
    parts: Vec<Part>,
    semaphore: Arc<Semaphore>,
}

impl R2Service {
    pub fn new(access_key: &str, secret_key: &str) -> Result<R2Service, S3Error> {
        let bucket = Bucket::new(
            &config().bucket.bucket_name,
            Region::Custom {
                region: "auto".to_string(),
                endpoint: config().bucket.r2_url.to_string()
            },
            Credentials::new(
                Some(access_key),
                Some(secret_key),
                None, None, None
            ).unwrap()
        )?
            .with_path_style()
            .deref()
            .to_owned();

        Ok(R2Service { bucket, active_uploads: Arc::new(RwLock::new(Vec::new())) })
    }

    pub async fn upload_part(
        &self,
        upload_id: &str,
        file_name: &str,
        chunk_number: u32,
        total_chunks: u32,
        chunk_data: Vec<u8>,
    ) -> Result<(), S3Error> {
        let semaphore = {
            let uploads = self.active_uploads.read().await;
            uploads
                .iter()
                .find(|a| a.upload_id == upload_id)
                .map(|a| a.semaphore.clone())
                .unwrap_or_else(|| Arc::new(Semaphore::new(3)))
        };
        let permit = semaphore.acquire_owned().await.unwrap();

        let mut attempts = 0;
        let completed_part = loop {
            attempts += 1;
            match self.bucket.put_multipart_chunk(
                chunk_data.clone(),
                file_name,
                chunk_number,
                upload_id,
                "application/octet-stream"
            ).await {
                Ok(part) => break part,
                Err(e) if attempts < 3 => {
                    log::warn!("Retrying chunk {} due to error: {}", chunk_number, e);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                },
                Err(e) => return Err(e),
            }
        };

        {
            let mut uploads = self.active_uploads.write().await;
            if let Some(upload) = uploads.iter_mut().find(|u| u.upload_id == upload_id) {
                upload.parts.push(completed_part);
            } else {
                uploads.push(ActiveUpload {
                    upload_id: upload_id.to_string(),
                    parts: vec![completed_part],
                    semaphore: Arc::new(Semaphore::new(3)),
                });
            }
        }

        if self.active_uploads.read().await
            .iter()
            .any(|upload| upload.upload_id == upload_id && upload.parts.len() == total_chunks as usize) {

            log::info!("All parts uploaded for file: {}", file_name);
            let parts = self.active_uploads.read().await
                .iter()
                .find(|upload| upload.upload_id == upload_id)
                .map(|upload| {
                    let mut parts = upload.parts.clone();
                    parts.sort_by_key(|part| part.part_number);
                    parts
                })
                .unwrap();

            log::info!("Completing multipart upload for file: {}", file_name);

            self.bucket.complete_multipart_upload(
                file_name,
                upload_id,
                parts
            ).await?;


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
        let initiation = self.bucket.initiate_multipart_upload(&file_name, "application/octet-stream").await;

        match initiation {
            Ok(initiation) => Ok(initiation.upload_id),
            Err(e) => Err(e.to_string()),
        }
    }
}