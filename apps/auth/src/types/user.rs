use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DBUserCreate {
    pub name: String,
    pub email: String,
    pub auth_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct RUserCreate {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserCreateRes {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRegenerateTokenRes {
    pub message: String,
}
