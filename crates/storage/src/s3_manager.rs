use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};

pub struct S3StorageManager {
    pub client: aws_sdk_s3::Client,
    pub bucket: String,
}

impl S3StorageManager {
    pub async fn new(access_key: String, secret_key: String, account_id: String, bucket: String) -> Self {
        let endpoint_url = format!("https://{}.r2.cloudflarestorage.com", account_id);

        let config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(Credentials::new(access_key, secret_key, None, None, "R2"))
            .region(Region::new("auto"))
            .endpoint_url(endpoint_url)
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true)
            .build();

        let client = aws_sdk_s3::Client::from_conf(s3_config);

        Self { client, bucket }
    }
}