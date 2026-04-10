use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ShareRequest {
    pub file_id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: u64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ShareResponse {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ShareDownloadRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ShareDownloadResponse {
    pub presigned_url: String,
    pub file_type: String,
    pub file_name: String,
    pub file_size: u64,
    pub created_at: String,
}

