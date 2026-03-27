use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ListFilesRequest {
    pub path: String,
    pub sort: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ListFileElement {
    pub id: String,
    pub file_name: String,
    pub file_size: i64,
    #[ts(type = "string")]
    pub created_at: DateTime<FixedOffset>,
    pub upload_completed: bool,
    pub file_type: String,
    pub path: String
}

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ListFilesResponse {
    pub files: Vec<ListFileElement>
}