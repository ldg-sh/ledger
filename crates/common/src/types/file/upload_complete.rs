use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct CompleteUploadRequest {
    pub file_id: String,
    pub upload_id: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct Part {
    pub part_number: u32,
    pub etag: String,
}