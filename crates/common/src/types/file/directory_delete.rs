use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct DeleteDirectoryRequest {
    pub path: String,
    pub directory_id: String
}