use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct RenameFileRequest {
    pub file_id: String,
    pub file_name: String,
}