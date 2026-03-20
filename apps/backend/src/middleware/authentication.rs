use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use std::future::{Ready, ready};
use actix_web::error::ErrorUnauthorized;
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
        println!("{:?}", req.cookies());
        let session = req.cookie("session");

        if session.is_none() {
            return ready(Err(ErrorUnauthorized("Unauthorized")));
        }

        if let Some(c) = session {
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
                Err(_) => ready(Err(ErrorUnauthorized("Invalid session token"))),
            }


        } else {
            ready(Err(ErrorUnauthorized("No session token found")))
        }
    }
}
