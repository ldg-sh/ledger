use sea_orm::entity::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime as ChronoDateTime, Utc};

pub struct TCreateFile {
    pub id: String,
    pub file_name: String,
    pub upload_id: String,
    pub owner_id: String,
    pub file_size: i64,
    pub created_at: DateTimeUtc,
    pub upload_completed: bool,
    pub file_type: String,
    pub path: String,
}

pub struct TCreateDirectory {
    pub id: String,
    pub file_name: String,
    pub upload_id: String,
    pub owner_id: String,
    pub created_at: DateTimeUtc,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TFileInfo {
    pub key: String,
    pub size: i64,
    pub last_modified: ChronoDateTime<Utc>,
    /// This might be none and generally doesn't matter but is a "nice to have"
    pub etag: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisFileMeta {
    pub info: TFileInfo,
    pub generation: i64
}
