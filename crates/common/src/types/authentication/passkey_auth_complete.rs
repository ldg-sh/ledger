use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct PasskeyAuthCompleteRequest {
    pub ticket: String,
    pub data: Value,
}