use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DirectoryRequest {
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DirectoryResponse {
    pub file_id: String
}