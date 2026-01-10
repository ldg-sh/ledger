use crate::modules::s3::s3_service::S3Service;
use anyhow::Result;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::delete_object::{DeleteObjectError, DeleteObjectOutput};

impl S3Service {
    pub async fn delete(
        &self,
        key: &str,
    ) -> Result<DeleteObjectOutput, SdkError<DeleteObjectError>> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
    }
}
