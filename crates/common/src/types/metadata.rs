use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataRequest {
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataResponse {
    pub filename: String,
    pub size: u64,
    pub content_type: String,
    pub user_id: String,
    pub path: String,
}