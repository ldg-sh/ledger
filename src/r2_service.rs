use std::collections::HashMap;
use std::ops::Deref;
use std::sync::RwLock;
use s3::{Bucket, Region};
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::serde_types::Part;

pub struct R2Service {
    bucket: Bucket,
    chunk_part_map: RwLock<HashMap<String, Vec<Part>>>,
}

impl R2Service {
    pub fn new(account_id: &str, access_token: &str, secret_key: &str) -> Result<R2Service, S3Error> {
        let bucket = Bucket::new(
            "ledger",
            Region::Custom {
                region: "auto".to_string(),
                endpoint: format!("https://{}.r2.cloudflarestorage.com", account_id)
            },
            Credentials::new(
                Some(access_token),
                Some(secret_key),
                None,
                None,
                None
            ).unwrap()
        )?
            .with_path_style()
            .deref()
            .to_owned();

        Ok(R2Service { bucket, chunk_part_map: RwLock::new(HashMap::new()) })
    }

    pub async fn upload_r2(
        &self,
        upload_id: &str,
        file_name: &str,
        chunk_number: u32,
        total_chunks: u32,
        chunk_data: Vec<u8>,
    ) -> Result<(), S3Error> {
        let completed_part = self.bucket.put_multipart_chunk(
            chunk_data,
            file_name,
            chunk_number,
            upload_id,
            "application/octet-stream"
        ).await?;

        self.chunk_part_map.write().unwrap()
            .entry(upload_id.to_string())
            .or_insert_with(Vec::new)
            .push(completed_part);
        
        if chunk_number == total_chunks {
            log::info!("Completing multipart upload for file: {}", file_name);
            self.bucket.complete_multipart_upload(
                file_name,
                upload_id,
                self.chunk_part_map.read().unwrap()
                    .get(upload_id)
                    .cloned()
                    .unwrap_or_else(Vec::new),
            ).await?;
            
            self.chunk_part_map.write().unwrap().remove(upload_id);
        }

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