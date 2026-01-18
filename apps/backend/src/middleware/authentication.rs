use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use std::future::{Ready, ready};

/// Placeholder for authenticated user.
/// TODO: Replace with proper authentication.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // TODO: Implement proper authentication
        // For now, extract user ID from X-User-Id header
        match req.headers().get("X-User-Id") {
            Some(user_id) => {
                match user_id.to_str() {
                    Ok(id) => ready(Ok(AuthenticatedUser { id: id.to_string() })),
                    Err(_) => ready(Err(actix_web::error::ErrorBadRequest("Invalid X-User-Id header"))),
                }
            }
            None => ready(Err(actix_web::error::ErrorUnauthorized("Missing X-User-Id header"))),
        }
    }
}
