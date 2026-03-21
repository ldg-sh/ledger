use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteUploadRequest {
    pub user_id: String,
    pub file_id: String,
}