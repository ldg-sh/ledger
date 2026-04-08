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
}

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct PresignedExplodedItem {
    pub id: String,
    pub file_name: String,
    pub virtual_path: String,
    pub presign_url: String,
}

#[derive(Deserialize, Serialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ZipRequest {
    pub item_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ExplodeResponse {
    pub items: Vec<PresignedExplodedItem>,
}