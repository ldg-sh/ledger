use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use std::future::{ready, Ready};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = req.headers();
        let user_id = headers
            .get("x-user-id")
            .and_then(|h| h.to_str().ok())
            .map(|h| h.to_string());

        let authenticated_user = AuthenticatedUser {
            id: user_id.unwrap_or("".to_string()),
        };
        
        ready(Ok(authenticated_user))
    }
}
