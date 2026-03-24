use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct CompleteUploadRequest {
    pub file_id: String,
}