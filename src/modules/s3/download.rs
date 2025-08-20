use crate::modules::s3::s3_service::S3Service;
use aws_sdk_s3::operation::get_object::GetObjectOutput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::operation::head_object::{HeadObjectError, HeadObjectOutput};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetMetadataResponse {
    content_size: i64,
    metadata: HashMap<String, String>,
    mime: String
}

impl S3Service {
    pub async fn get_metadata(&self, file_name: &str) -> Result<GetMetadataResponse, Error> {
        let response = match self.client.head_object()
            .key(file_name)
            .bucket(&self.bucket)
            .send()
            .await {
            Ok(res) => res,
            Err(error) => {
                println!("{:?}", error.raw_response().unwrap().body());
                return Err(Error::new(std::io::ErrorKind::Other, error.to_string()));
            }
        };

        let size = match response.content_length() {
            Some(size) => size,
            None => return Err(Error::other( "Failed to get response content size.")),
        };

        let metadata = match response.metadata() {
            Some(metadata) => metadata,
            None => return Err(Error::other("Failed to get response content metadata." )),
        }.to_owned();

        let mime = match response.content_type() {
            Some(c) => c,
            None => return Err(Error::other("Failed to get response content type." )),
        };

        Ok(GetMetadataResponse {
            content_size: size,
            metadata,
            mime: mime.to_string()
        })
    }

    pub async fn download_part(&self, key: &str, start: u64, end: u64) -> Result<GetObjectOutput, Error> {
        let range = format!("bytes={}-{}", start, end);

        self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .range(&range)
            .send()
            .await
            .map_err(|e| Error::other(e.to_string()))
    }

    pub async fn download_file(&self, key: &str) -> Result<GetObjectOutput, Error> {
        self.client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| Error::other(e.to_string()))
    }
}
