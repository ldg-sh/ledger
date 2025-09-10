use crate::modules::s3::s3_service::S3Service;
use anyhow::Result;
use aws_sdk_s3::primitives::ByteStream;

impl S3Service {
    pub async fn create_team_folder(&self, team_id: &str) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(format!("teams/{team_id}/"))
            .body(ByteStream::from_static(b""))
            .send()
            .await?;
        Ok(())
    }
}
