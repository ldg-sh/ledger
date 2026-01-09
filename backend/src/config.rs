use std::env;
use std::sync::OnceLock;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EnvConfig {
    pub bucket: BucketDetails,
    pub postgres: PostgresDetails,
    pub grpc: Grpc,
    pub redis: RedisDetails,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Grpc {
    pub url: String,
    pub auth_key: String,
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
            grpc: Grpc {
                url: Self::get_env("GRPC_URL"),
                auth_key: Self::get_env("GRPC_AUTH_KEY"),
            },
            redis: RedisDetails {
                redis_url: Self::get_env("REDIS_URL"),
            },
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
