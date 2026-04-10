use chrono::{DateTime, FixedOffset};
#[cfg(feature = "ssr")]
use sea_orm::{FromQueryResult};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
#[cfg_attr(feature = "ssr", derive(FromQueryResult))]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ExplodedItem {
    pub id: String,
    pub file_name: String,
    pub virtual_path: String,
    pub file_size: i64,
    #[ts(type = "string")]
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct PresignedExplodedItem {
    pub id: String,
    pub file_name: String,
    pub virtual_path: String,
    pub presign_url: String,
    pub size: i64,
    #[ts(type = "string")]
    pub created_at: DateTime<FixedOffset>,
}

#[derive(Deserialize, Serialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ZipRequest {
    pub item_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ExplodeResponse {
    pub items: Vec<PresignedExplodedItem>,
}