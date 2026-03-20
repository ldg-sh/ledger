use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use std::future::{Ready, ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::config::config;
use crate::util::auth::UserClaims;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let cookie = req.cookie("session");

        if let Some(c) = cookie {
            let secret = &config().auth.jwt_secret;

            let token_data = decode::<UserClaims>(
                c.value(),
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            );

            match token_data {
                Ok(data) => {
                    let user_id = data.claims.user_id.to_string();
                    ready(Ok(AuthenticatedUser { id: user_id }))
                },
                Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid session token"))),
            }


        } else {
            ready(Err(actix_web::error::ErrorUnauthorized("No session token found")))
        }
    }
}
