use crate::config::config;
use crate::modules::s3::upload::ActiveUpload;
use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::error::ProvideErrorMetadata;
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
            .force_path_style(true)
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

    pub async fn ping(&self) -> Result<(), Error> {
        self.client
            .head_bucket()
            .bucket(&self.bucket)
            .send()
            .await
            .map(|_| ())
            .map_err(|err| {
                Error::other(format!(
                    "Failed to reach bucket '{}': {}",
                    self.bucket,
                    err.message().unwrap_or("unknown error")
                ))
            })
    }

    pub async fn ensure_bucket(&self) -> Result<(), Error> {
        match self.client.head_bucket().bucket(&self.bucket).send().await {
            Ok(_) => Ok(()),
            Err(e) => {
                // If bucket doesn't exist (or any error), try to create it
                log::warn!(
                    "head_bucket failed for '{}': {} — attempting create_bucket",
                    self.bucket,
                    e.message().unwrap_or("unknown error")
                );
                self.client
                    .create_bucket()
                    .bucket(&self.bucket)
                    .send()
                    .await
                    .map(|_| ())
                    .map_err(|err| {
                        Error::other(format!(
                            "Failed to create bucket '{}': {}",
                            self.bucket,
                            err.message().unwrap_or("unknown error")
                        ))
                    })
            }
        }
    }

    pub async fn list_directories(&self, prefix: &str) -> Result<Vec<String>, Error> {
        let normalized_prefix = if prefix.is_empty() {
            String::new()
        } else if prefix.ends_with('/') {
            prefix.to_string()
        } else {
            format!("{}/", prefix)
        };

        let response = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(&normalized_prefix)
            .delimiter("/")
            .send()
            .await
            .map_err(|err| {
                Error::other(format!(
                    "Failed to list directories in '{}': {}",
                    normalized_prefix,
                    err.message().unwrap_or("unknown error")
                ))
            })?;

        let directories = response
            .common_prefixes()
            .iter()
            .filter_map(|cp| cp.prefix())
            .map(|s| {
                s.strip_prefix(&normalized_prefix)
                    .unwrap_or(s)
                    .trim_end_matches('/')
                    .to_string()
            })
            .collect();

        Ok(directories)
    }
}
