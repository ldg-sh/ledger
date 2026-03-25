use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct InitUploadRequest {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub path: String,
    pub part_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct InitUploadResponse {
    pub file_id: String,
    pub upload_urls: Vec<String>,
    pub upload_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct InitUploadInternalRequest {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub user_id: String,
    pub path: String,
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct InitUploadInternalResponse {
    pub upload_id: String,
}