use crate::types::redis::RedisKeyTypes;

pub fn create_file_upload_redis_key(name: &str, token: &str) -> String {
    let redis_key_type = RedisKeyTypes::FileCreate.to_string();
    format!("{}:{}:{}", redis_key_type, name, token)
}

pub fn make_redis_key<K: ToString>(key_type: K, parts: &[&str]) -> String {
    let mut key = key_type.to_string();
    for p in parts {
        key.push(':');
        key.push_str(p);
    }
    key
}
