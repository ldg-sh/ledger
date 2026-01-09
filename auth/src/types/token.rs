use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub enum TokenType {
    User,
    Admin,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::User => write!(f, "user"),
            TokenType::Admin => write!(f, "admin"),
        }
    }
}

pub fn construct_token(user_id: &str, api_key: &str) -> String {
    BASE64_STANDARD.encode(format!("{user_id}.{api_key}"))
}
