use crate::types::error::AuthError;
use common::types::user_claims::UserClaims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use worker::*;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,

}

pub async fn get_authenticated_user(req: &Request, env: &Env) -> Result<AuthenticatedUser, AuthError> {
    let jwt_secret = env.var("JWT_SECRET").map_err(|err| AuthError::EnvError(err))?.to_string();

    let cookie_header = req.headers().get("Cookie")
        .map_err(AuthError::EnvError)?
        .ok_or(AuthError::MissingToken)?;

    let session_token = cookie_header
        .split(';')
        .find(|s| s.trim().starts_with("session="))
        .map(|s| s.trim().trim_start_matches("session="))
        .ok_or_else(|| Error::from("Unauthorized"))
        .map_err(|_| AuthError::InvalidToken)?;

    let token_data = decode::<UserClaims>(
        session_token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ).map_err(|_| Error::from("Invalid session token")).map_err(|_| AuthError::InvalidToken)?;

    Ok(AuthenticatedUser {
        id: token_data.claims.user_id.to_string(),
    })
}