use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct PasskeyInitRequest {
    pub username: String,
    pub existing_id: Option<String>
}

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct PasskeyInitResponse {
    pub user_id: String,
    pub response: Value
}