use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RenameFileRequest {
    pub file_id: String,
    pub file_name: String,
}