use std::env;
use std::sync::OnceLock;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EnvConfig {
    pub bucket: BucketDetails,
    pub postgres: PostgresDetails,
    pub redis: RedisDetails,
    pub auth: AuthDetails,
    pub port: u16,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BucketDetails {
    pub bucket_name: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_url: String,
    pub s3_region: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PostgresDetails {
    pub postgres_uri: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RedisDetails {
    pub redis_url: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AuthDetails {
    pub jwt_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_callback_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        EnvConfig {
            bucket: BucketDetails {
                bucket_name: Self::get_env("S3_BUCKET_NAME"),
                s3_access_key: Self::get_env("S3_ACCESS_KEY"),
                s3_secret_key: Self::get_env("S3_SECRET_KEY"),
                s3_url: Self::get_env("S3_URL"),
                s3_region: Self::get_env("S3_BUCKET_REGION"),
            },
            postgres: PostgresDetails {
                postgres_uri: Self::get_env("POSTGRES_URI"),
            },
            redis: RedisDetails {
                redis_url: Self::get_env("REDIS_URL"),
            },
            auth: AuthDetails {
                jwt_secret: Self::get_env("JWT_SECRET"),
                google_client_id: Self::get_env("GOOGLE_CLIENT_ID"),
                google_client_secret: Self::get_env("GOOGLE_CLIENT_SECRET"),
                google_callback_url: Self::get_env("GOOGLE_CALLBACK_URL"),
                github_client_id: Self::get_env("GITHUB_CLIENT_ID"),
                github_client_secret: Self::get_env("GITHUB_CLIENT_SECRET"),
            },
            port: Self::get_env("PORT").parse().expect("PORT must be a valid u16"),
        }
    }

    fn get_env(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not set", key))
    }
}

pub static CONFIG: OnceLock<EnvConfig> = OnceLock::new();

#[allow(dead_code)]
pub fn config() -> &'static EnvConfig {
    CONFIG.get().expect("Not initialized")
}
