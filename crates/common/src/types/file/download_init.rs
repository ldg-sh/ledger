use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct InitDownloadRequest {
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct InitDownloadResponse {
    pub download_url: String,
}