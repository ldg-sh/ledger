use crate::ProviderConfiguration;
use actix_web::dev::{Payload, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::{Error, FromRequest, HttpRequest};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};
use actix_web::body::MessageBody;
use actix_web::middleware::Next;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub exp: i64,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let session_token = req.cookie("session").map(|c| c.value().to_string());

        let token = match session_token {
            Some(t) => t,
            None => {
                return ready(Err(actix_web::error::ErrorUnauthorized(
                    "No session cookie found",
                )));
            }
        };

        let secret = req
            .app_data::<Data<ProviderConfiguration>>()
            .unwrap()
            .jwt_secret
            .clone();

        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::new(Algorithm::HS256);

        match decode::<Claims>(&token, &decoding_key, &validation) {
            Ok(token_data) => ready(Ok(AuthenticatedUser {
                id: token_data.claims.user_id,
            })),
            Err(err) => {
                println!("Token decoding error: {:?}", err);
                ready(Err(actix_web::error::ErrorUnauthorized(
                    "Invalid or expired token",
                )))
            }
        }
    }
}

pub async fn reject_bypassed_traffic(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let origin_secret = req
        .app_data::<Data<ProviderConfiguration>>()
        .unwrap()
        .origin_secret
        .clone();

    if let Some(header_value) = req.headers().get("X-Origin-Secret") {
        if let Ok(secret_str) = header_value.to_str() {
            if secret_str == origin_secret {
                return next.call(req).await;
            }
        }
    }

    Err(actix_web::error::ErrorForbidden(
        "Direct access to the authentication server is forbidden.",
    ))
}