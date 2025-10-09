use crate::config::config;
use crate::ledger::ValidationRequest;
use crate::ledger::authentication_client::AuthenticationClient;
use actix_web::{Error, FromRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures_util::future::LocalBoxFuture;
use tonic::Request as GrpcRequest;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct Authentication;

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

        let grpc_client = match req.app_data::<actix_web::web::Data<tonic::transport::Channel>>() {
            Some(c) => c.get_ref().clone(),
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

            let mut client = AuthenticationClient::new(grpc_client);
            let mut grpc_req = GrpcRequest::new(ValidationRequest {
                token: token.clone(),
            });

            let v: MetadataValue<Ascii> =
                config()
                    .grpc
                    .auth_key
                    .parse()
                    .map_err(|e: InvalidMetadataValue| {
                        actix_web::error::ErrorUnauthorized(e.to_string())
                    })?;

            grpc_req.metadata_mut().insert("authorization", v);

            let resp = client
                .validate_authentication(grpc_req)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            let inner = resp.into_inner();
            if !inner.is_valid {
                return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
            }

            let authenticated_user = AuthenticatedUser {
                user_id: inner.user_id,
            };

            req.extensions_mut().insert(authenticated_user);

            srv.call(req).await
        })
    }
}
