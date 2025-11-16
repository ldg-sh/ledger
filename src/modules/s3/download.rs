use crate::modules::s3::s3_service::S3Service;
use anyhow::Result;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectOutput;
use aws_sdk_s3::operation::head_object::{HeadObjectError, HeadObjectOutput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use crate::config::config;
use crate::types::file::TFileInfo;
use chrono::{DateTime as ChronoDateTime, Utc};

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
    ) -> Result<HeadObjectOutput, SdkError<HeadObjectError>> {
        self.client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
    }

    pub async fn list_files(
        &self,
        cursor: Option<String>
    ) -> Result<(Vec<TFileInfo>, Option<String>)> {
        let mut req = self.client
            .list_objects_v2()
            .bucket(config().clone().bucket.bucket_name)
            .prefix("");

        if let Some(token) = cursor {
            req = req.continuation_token(token);
        }

        let objs = req.send().await?;

        let parsed: Vec<TFileInfo> = objs.contents
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|obj| {
                // extract required fields
                let key = obj.key.as_ref()?.clone();
                let size = obj.size?;
                if size == 0 {
                    return None;
                }

                let aws_dt = obj.last_modified.as_ref()?;
                let last_modified = aws_dt.to_string()
                    .parse::<ChronoDateTime<Utc>>()
                    .ok()?;

                Some(TFileInfo {
                    key,
                    size,
                    last_modified,
                    etag: obj.e_tag.clone(),
                })
            })
            .collect();

        // This is the continuation token for next page (if any)
        let next_cursor = objs
            .next_continuation_token
            .map(|s| s.to_string());

        Ok((parsed, next_cursor))
    }

    pub async fn download_part(
        &self,
        key: &str,
        start: u64,
        end: u64,
    ) -> Result<GetObjectOutput, Error> {
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
