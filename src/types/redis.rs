use strum_macros::{EnumString, Display};

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum RedisKeyTypes {
    FileUpload
}
