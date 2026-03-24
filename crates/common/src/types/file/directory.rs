use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct DirectoryRequest {
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct DirectoryResponse {
    pub file_id: String
}