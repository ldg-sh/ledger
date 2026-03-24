use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct DeleteFilesRequest {
    pub file_ids: Vec<String>,
}