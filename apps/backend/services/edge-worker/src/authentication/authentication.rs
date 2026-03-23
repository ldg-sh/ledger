use crate::types::error::AuthError;
use common::types::user_claims::UserClaims;
use jsonwebtoken::{decode, DecodingKey, Validation};
use worker::*;
use crate::types::configuration::Configuration;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,

}

pub async fn get_authenticated_user(req: &Request, ctx: &RouteContext<Configuration>) -> Result<AuthenticatedUser, AuthError> {
    let config = &ctx.data;

    let jwt_secret = config.jwt_secret.clone();

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