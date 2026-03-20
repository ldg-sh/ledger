use serde::{Deserialize, Serialize};
use crate::config::config;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub user_id: String,
    pub exp: i64,
}

pub fn generate_access_token(
    user_id: &str
) -> String {
    let expiration = chrono::Utc::now() + chrono::Duration::minutes(15);

    let user_claims = UserClaims {
        user_id: user_id.to_string(),
        exp: expiration.timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &user_claims,
        &jsonwebtoken::EncodingKey::from_secret(config().auth.jwt_secret.as_ref()),
    ).unwrap()
}