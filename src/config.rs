use dotenv;
use std::env;
use std::sync::OnceLock;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EnvConfig {
    pub bucket: BucketDetails
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BucketDetails {
    pub bucket_name: String,
    pub r2_access_key: String,
    pub r2_secret_key: String,
    pub r2_url: String,

}

impl EnvConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        
        EnvConfig {
            bucket: BucketDetails {
                bucket_name: Self::get_env("R2_BUCKET_NAME"),
                r2_access_key: Self::get_env("R2_ACCESS_KEY"),
                r2_secret_key: Self::get_env("R2_SECRET_KEY"),
                r2_url: Self::get_env("R2_URL"),
            }
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