use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};

pub struct S3StorageManager {
    pub client: aws_sdk_s3::Client,
    pub bucket: String,
}

impl S3StorageManager {
    pub async fn new_s3(access_key: String, secret_key: String, bucket: String, endpoint: String) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(Credentials::new(access_key, secret_key, None, None, "S3"))
            .region(Region::new("auto"))
            .endpoint_url(endpoint)
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .build();

        let client = aws_sdk_s3::Client::from_conf(s3_config);

        Self { client, bucket }
    }
}