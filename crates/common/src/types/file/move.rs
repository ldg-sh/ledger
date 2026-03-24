use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct MoveFilesRequest {
    pub file_ids: Vec<String>,
    pub destination_path: String,
}