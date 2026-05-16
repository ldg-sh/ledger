use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileShare {
    pub file_id: String,
    pub file_name: String,
    pub owner_id: String,
    pub file_size: u64,
    pub file_type: String,
    pub created_at: String,
}
