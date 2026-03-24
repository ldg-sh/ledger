use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct MetadataRequest {
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct MetadataResponse {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub path: String,
}