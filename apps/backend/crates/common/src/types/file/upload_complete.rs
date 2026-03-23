use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteUploadRequest {
    pub file_id: String,
}