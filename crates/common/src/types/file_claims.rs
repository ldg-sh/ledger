use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileClaims {
    pub file_id: String,
    pub file_name: String,
    pub owner_id: String,
    pub file_type: String
}
