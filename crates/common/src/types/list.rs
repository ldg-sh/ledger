use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListFilesRequest {
    pub path: String,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub user_id: String
}

#[derive(Serialize, Deserialize)]
pub struct ListFileElement {
    pub id: String,
    pub file_name: String,
    pub file_size: i64,
    pub created_at: DateTime<FixedOffset>,
    pub upload_completed: bool,
    pub file_type: String,
    pub path: String
}

#[derive(Serialize, Deserialize)]
pub struct ListFilesResponse {
    pub files: Vec<ListFileElement>
}