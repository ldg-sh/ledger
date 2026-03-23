use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CopyFilesRequest {
    pub file_ids: Vec<String>,
    pub destination_path: String,
    pub user_id: String
}

#[derive(Serialize, Deserialize)]
pub struct CopyFilesResponse {
    pub file_ids: Vec<String>
}