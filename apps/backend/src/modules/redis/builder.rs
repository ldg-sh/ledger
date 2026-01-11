pub struct RedisKeyBuilder {}

impl RedisKeyBuilder {
    pub fn file_log_key(file_key: &str) -> String {
        format!("file:meta:{file_key}")
    }

    pub fn scan_token() -> String {
        "file:scan".to_string()
    }

    pub fn generation_key() -> String {
        "file:scan:generation".to_string()
    }

    pub fn generation_members_key(generation: i64) -> String {
        format!("file:gen:{generation}")
    }
}
