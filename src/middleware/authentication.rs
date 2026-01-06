use crate::context::AppContext;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::LocalBoxFuture;
use std::sync::Arc;

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready, Payload};
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

        let grpc_service = match req.app_data::<actix_web::web::Data<Arc<AppContext>>>() {
            Some(c) => {
                let ctx = Arc::clone(c.get_ref());
                Arc::clone(&ctx.grpc_service)
            }
            None => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorInternalServerError(
                        "gRPC client not configured",
                    ))
                });
            }
        };

        let srv = self.service.clone();

        Box::pin(async move {
            let auth = fut
                .await
                .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;

            let token = auth.token().to_string();

            let resp = grpc_service
                .validate_authentication(&token)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            log::info!("Authentication response: is_valid={:?}", resp);

            if !resp.is_valid {
                return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
            }

            req.extensions_mut().insert(AuthenticatedUser {
                id: resp.user_id.clone(),
            });

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
