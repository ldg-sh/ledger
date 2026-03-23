use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadRequest {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub user_id: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadResponse {
    pub file_id: String,
    pub upload_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitUploadInternalRequest {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub user_id: String,
    pub path: String,
    pub file_id: String,
}