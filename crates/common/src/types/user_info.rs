use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct UserInfoRequest {
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct UserInfoResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
}