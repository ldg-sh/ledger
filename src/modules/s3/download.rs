use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_s3::operation::head_object::{HeadObjectError, HeadObjectOutput};
use serde::{Deserialize, Serialize};
use crate::modules::s3::s3_service::S3Service;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetMetadataResponse {
    content_size: i64,
    metadata: HashMap<String, String>,
}

impl S3Service {
    pub async fn get_metadata(&self, file_name: &str) -> Result<GetMetadataResponse, Error> {
        let response = self.client.head_object()
            .key(file_name)
            .bucket(&self.bucket)
            .send()
            .await
            .map_err(|e| {
                Error::new(ErrorKind::Other, format!("Failed to get head uwu: {e}"))
            })?;

        let size = match response.content_length() {
            Some(size) => size,
            None => return Err(Error::new(ErrorKind::Other, "Failed to get response content size." )),
        };

        let metadata = match response.metadata() {
            Some(metadata) => metadata,
            None => return Err(Error::new(ErrorKind::Other, "Failed to get response content metadata." )),
        }.to_owned();
        
        Ok(GetMetadataResponse {
            content_size: size,
            metadata,
        })
    }

    pub async fn download_part(&self, key: &str, start: u64, end: u64) -> Result<GetObjectOutput, Error> {
        let range = format!("bytes={}-{}", start, end);
        let resp = self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .range(&range)
            .send()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()));
        Ok(resp?)
    }
}