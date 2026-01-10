use std::env;

/// Extension trait for environment variable handling
pub trait ConfigExt {
    fn get_env(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("Environment variable {} not set", key))
    }

    fn get_env_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    fn get_env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }
}

/// Blanket implementation so any config struct can use these helpers
impl<T> ConfigExt for T {}
