use crate::modules::s3::s3_service::S3Service;
use crate::util::strings::compound_team_file;
use anyhow::Result;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_s3::operation::head_object::{HeadObjectError, HeadObjectOutput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetMetadataResponse {
    pub content_size: i64,
    pub metadata: Option<HashMap<String, String>>,
    pub mime: String,
}

impl S3Service {
    pub async fn get_metadata(
        &self,
        key: &str,
        team_id: &str,
    ) -> Result<HeadObjectOutput, SdkError<HeadObjectError>> {
        self.client
            .head_object()
            .bucket(&self.bucket)
            .key(compound_team_file(team_id, key))
            .send()
            .await
    }

    pub async fn download_part(
        &self,
        key: &str,
        team_id: &str,
        start: u64,
        end: u64,
    ) -> Result<GetObjectOutput, Error> {
        let range = format!("bytes={}-{}", start, end);

        self.client
            .get_object()
            .bucket(&self.bucket)
            .key(compound_team_file(team_id, key))
            .range(&range)
            .send()
            .await
            .map_err(|e| Error::other(e.to_string()))
    }

    pub async fn download_file(&self, key: &str, team_id: &str) -> Result<GetObjectOutput, Error> {
        self.client
            .get_object()
            .bucket(&self.bucket)
            .key(compound_team_file(team_id, key))
            .send()
            .await
            .map_err(|e| Error::other(e.to_string()))
    }
}
