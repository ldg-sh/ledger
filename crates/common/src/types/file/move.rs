use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MoveFilesRequest {
    pub file_ids: Vec<String>,
    pub destination_path: String,
}