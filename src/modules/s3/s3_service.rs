use crate::config::config;
use crate::modules::s3::upload::ActiveUpload;
use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use std::io::Error;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct S3Service {
    pub bucket: String,
    pub client: Client,
    pub(super) active_uploads: Arc<RwLock<Vec<ActiveUpload>>>,
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
                    .build(),
            )
            .build();

        let client = Client::from_conf(config);
        Ok(S3Service {
            bucket: bucket.to_string(),
            client,
            active_uploads: Arc::new(RwLock::new(Vec::new())),
        })
    }
}
