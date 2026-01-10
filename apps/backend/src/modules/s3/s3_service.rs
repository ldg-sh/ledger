use crate::config::config;
use crate::modules::s3::upload::ActiveUpload;
use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::primitives::ByteStream;
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

    pub async fn delete_file(&self, key: &str) -> Result<(), Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map(|_| ())
            .map_err(|err| {
                Error::other(format!(
                    "Failed to delete file '{}': {}",
                    key,
                    err.message().unwrap_or("unknown error")
                ))
            })
    }

    pub async fn delete_multiple_files(&self, keys: Vec<String>) -> Result<(), Error> {
        let objects = keys
            .iter()
            .map(|key| {
                aws_sdk_s3::types::ObjectIdentifier::builder()
                    .key(key)
                    .build()
            })
            .collect::<Vec<_>>();

        self.client
            .delete_objects()
            .bucket(&self.bucket)
            .delete(
                aws_sdk_s3::types::Delete::builder()
                    .set_objects(Some(
                        objects
                            .iter()
                            .filter(|obj| obj.is_ok())
                            .map(|obj| obj.as_ref().unwrap())
                            .map(|obj| obj.clone().to_owned())
                            .collect(),
                    ))
                    .build()
                    .expect("REASON"),
            )
            .send()
            .await
            .map(|_| ())
            .map_err(|err| {
                Error::other(format!(
                    "Failed to delete multiple files: {}",
                    err.message().unwrap_or("unknown error")
                ))
            })
    }

    pub async fn create_blank_directory(&self, dir_name: &str) -> Result<(), Error> {
        let dir_key = if dir_name.ends_with('/') {
            dir_name.to_string()
        } else {
            format!("{}/", dir_name)
        };

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&dir_key)
            .body(ByteStream::from_static(b""))
            .send()
            .await
            .map(|_| ())
            .map_err(|err| {
                Error::other(format!(
                    "Failed to create directory '{}': {}",
                    dir_key,
                    err.message().unwrap_or("unknown error")
                ))
            })
    }

    pub async fn delete_directory(&self, dir_name: &str) -> Result<(), Error> {
        let dir_key = if dir_name.ends_with('/') {
            dir_name.to_string()
        } else {
            format!("{}/", dir_name)
        };

        let objects_to_delete = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(&dir_key)
            .send()
            .await
            .map_err(|err| {
                Error::other(format!(
                    "Failed to list objects for deletion in '{}': {}",
                    dir_key,
                    err.message().unwrap_or("unknown error")
                ))
            })?
            .contents()
            .iter()
            .filter_map(|obj| obj.key().map(|k| k.to_string()))
            .collect::<Vec<String>>();

        if objects_to_delete.is_empty() {
            return Ok(());
        }

        for chunk in objects_to_delete.chunks(1000) {
            let objects = chunk
                .iter()
                .map(|key| {
                    aws_sdk_s3::types::ObjectIdentifier::builder()
                        .key(key)
                        .build()
                })
                .collect::<Vec<_>>();

            let res = self
                .client
                .delete_objects()
                .bucket(&self.bucket)
                .delete(
                    aws_sdk_s3::types::Delete::builder()
                        .set_objects(Some(
                            objects
                                .iter()
                                .filter(|obj| obj.is_ok())
                                .map(|obj| obj.as_ref().unwrap())
                                .map(|obj| obj.clone().to_owned())
                                .collect(),
                        ))
                        .build()
                        .expect("Failed to build delete objects"),
                )
                .send()
                .await
                .map_err(|err| {
                    Error::other(format!(
                        "Failed to delete objects in chunk: {}",
                        err.message().unwrap_or("unknown error")
                    ))
                })?;

            if res.errors().len() > 0 {
                return Err(Error::other(format!(
                    "Errors occurred while deleting objects in chunk: {:?}",
                    res.errors()
                )));
            }
        }

        Ok(())
    }

    pub async fn copy_file(&self, source_key: &str, destination_key: &str) -> Result<(), Error> {
        let copy_source = format!("{}/{}", self.bucket, source_key);

        self.client
            .copy_object()
            .bucket(&self.bucket)
            .key(destination_key)
            .copy_source(copy_source)
            .send()
            .await
            .map(|_| ())
            .map_err(|err| {
                Error::other(format!(
                    "Failed to copy file from '{}' to '{}': {}",
                    source_key,
                    destination_key,
                    err.message().unwrap_or("unknown error")
                ))
            })
    }

    pub async fn move_file(&self, source_key: &str, destination_key: &str) -> Result<(), Error> {
        self.copy_file(source_key, destination_key).await?;
        self.delete_file(source_key).await?;
        Ok(())
    }

    pub async fn move_multiple_files(
        &self,
        source_keys: Vec<String>,
        destination_keys: Vec<String>,
    ) -> Result<(), Error> {
        if source_keys.len() != destination_keys.len() {
            return Err(Error::other("Source and destination keys length mismatch"));
        }

        for (source_key, destination_key) in source_keys.iter().zip(destination_keys.iter()) {
            self.copy_file(source_key, destination_key).await?;
            self.delete_file(source_key).await?;
        }

        Ok(())
    }

    pub async fn copy_multiple_files(
        &self,
        source_keys: Vec<String>,
        destination_keys: Vec<String>,
    ) -> Result<(), Error> {
        if source_keys.len() != destination_keys.len() {
            return Err(Error::other("Source and destination keys length mismatch"));
        }

        for (source_key, destination_key) in source_keys.iter().zip(destination_keys.iter()) {
            self.copy_file(source_key, destination_key).await?;
        }

        Ok(())
    }
}
