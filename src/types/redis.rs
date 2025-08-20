use strum_macros::{EnumString, Display};
use crate::util::strings;

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum RedisKeyTypes {
    FileUpload,
    FileCreate
}

impl RedisKeyTypes {
    // Wrapper in case method changes :D
    pub fn make(&self, parts: &[&str]) -> String {
        strings::make_redis_key(self.to_string(), parts)
    }
}
