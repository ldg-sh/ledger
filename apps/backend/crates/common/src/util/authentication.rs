use crate::types::user_claims::UserClaims;

pub fn generate_access_token(
    user_id: &str,
    jwt_secret: &str,
) -> String {
    let expiration = chrono::Utc::now() + chrono::Duration::minutes(15);

    let user_claims = UserClaims {
        user_id: user_id.to_string(),
        exp: expiration.timestamp(),
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &user_claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
    ).unwrap()
}