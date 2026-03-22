use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitDownloadRequest {
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitDownloadResponse {
    pub download_url: String,
}