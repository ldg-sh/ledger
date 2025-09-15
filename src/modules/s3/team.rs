use crate::modules::s3::s3_service::S3Service;
use crate::types::error::AppError;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::head_object::HeadObjectError;

impl S3Service {

    pub async fn create_team_folder(&self, team_id: &str) -> Result<(), AppError> {
        // Check if already exists.
        let exists = match self.team_folder_exists(team_id).await {
            Ok(i) => i,
            Err(e) => {
                return Err(AppError::Internal(e.to_string()))
            },
        };

        if exists {
            return Ok(());
        }

        match self.client
                .put_object()
                .bucket(&self.bucket)
                .key(format!("teams/{team_id}/"))
                .body(ByteStream::from_static(b""))
                .send()
                .await {
                Ok(_) => {},
                Err(e) => {
                    return Err(AppError::Internal(e.to_string()))
                },
            }

        Ok(())
    }

    pub async fn team_folder_exists(&self, team_id: &str) -> Result<bool, SdkError<HeadObjectError>> {
        let key = format!("teams/{team_id}/");
        match self.client.head_object().bucket(&self.bucket).key(&key).send().await {
            Ok(_) => Ok(true),
            Err(e) => {
                if let SdkError::ServiceError(service_error) = &e && matches!(service_error.err(), HeadObjectError::NotFound(_)) {
                    return Ok(false);
                }
                Err(e)
            }
        }
    }
}
