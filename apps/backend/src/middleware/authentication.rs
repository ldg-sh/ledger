use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::LocalBoxFuture;

use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct Authentication;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: String,
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let mut payload = req.take_payload();
        let fut = BearerAuth::from_request(req.request(), &mut payload);

        req.set_payload(payload);

        let srv = self.service.clone();

        Box::pin(async move {
            let auth = fut
                .await
                .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

            let token = auth.token().to_string();

            // TODO: Implement proper token validation
            // For now, accept any non-empty bearer token
            if token.is_empty() {
                return Err(actix_web::error::ErrorUnauthorized("Empty token"));
            }

            // Use the token as the user ID for now
            req.extensions_mut().insert(AuthenticatedUser { id: token });

            srv.call(req).await
        })
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<AuthenticatedUser>() {
            Some(user) => ready(Ok(user.clone())),
            None => ready(Err(actix_web::error::ErrorInternalServerError(
                "AuthenticatedUser missing from request extensions",
            ))),
        }
    }
}
