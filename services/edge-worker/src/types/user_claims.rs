use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub user_id: String,
    pub exp: i64,
}
