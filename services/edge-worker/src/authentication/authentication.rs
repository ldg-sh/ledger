use worker::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::types::user_claims::UserClaims;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,

}

pub async fn get_authenticated_user(req: &Request, env: &Env) -> Result<AuthenticatedUser> {
    let jwt_secret = env.var("JWT_SECRET")?.to_string();

    let cookie_header = req.headers().get("Cookie")?.unwrap_or_default();

    let session_token = cookie_header
        .split(';')
        .find(|s| s.trim().starts_with("session="))
        .map(|s| s.trim().trim_start_matches("session="))
        .ok_or_else(|| Error::from("Unauthorized"))?;

    let token_data = decode::<UserClaims>(
        session_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| Error::from("Invalid session token"))?;

    Ok(AuthenticatedUser {
        id: token_data.claims.user_id.to_string(),
    })
}