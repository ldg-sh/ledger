use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct CopyFilesRequest {
    pub file_ids: Vec<String>,
    pub destination_path: String,
}

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct CopyFilesResponse {
    pub file_ids: Vec<String>
}