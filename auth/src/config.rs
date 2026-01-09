use std::env;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct EnvConfig {
    pub port: i32,
    pub db_url: String,
    pub admin_key: String,
    pub resend_key: String,
    pub grpc: GrpcConfig,
}

#[derive(Clone, Debug)]
pub struct GrpcConfig {
    pub port: i32,
    pub auth_key: String,
}

impl EnvConfig {
    fn get_env(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not set", key))
    }

    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let db_url: String = Self::get_env("POSTGRES_URI");
        let resend_key: String = Self::get_env("RESEND_KEY");

        EnvConfig {
            port: Self::get_env("PORT").parse().unwrap_or(8081),
            db_url,
            admin_key: Self::get_env("ADMIN_KEY"),
            resend_key,
            grpc: GrpcConfig {
                port: Self::get_env("GRPC_PORT").parse().unwrap_or(50051),
                auth_key: Self::get_env("GRPC_AUTH_KEY"),
            },
        }
    }
}

pub static CONFIG: OnceLock<EnvConfig> = OnceLock::new();

#[allow(dead_code)]
pub fn config() -> &'static EnvConfig {
    CONFIG.get().expect("Not initialized")
}
