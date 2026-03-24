use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteFilesRequest {
    pub file_ids: Vec<String>,
}