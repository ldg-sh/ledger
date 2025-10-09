use crate::config::config;
use crate::ledger::GetUserTeamRequest;
use crate::ledger::authentication_client::AuthenticationClient;
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use tonic::Request as GrpcRequest;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::metadata::{Ascii, MetadataValue};

use crate::middleware::authentication::AuthenticatedUser;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use std::future::{Ready, ready};
use std::rc::Rc;

pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthorizationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
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
        let payload = req.take_payload();

        let team_id = req.match_info().get("team").unwrap_or("").to_string();

        if team_id.is_empty() {
            return Box::pin(async {
                Err(actix_web::error::ErrorBadRequest(
                    "Missing team or key in path",
                ))
            });
        }

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
            let user = req
                .extensions()
                .get::<AuthenticatedUser>()
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("Not authenticated"))?
                .clone();

            let mut client = AuthenticationClient::new(grpc_client);
            let mut grpc_req = GrpcRequest::new(GetUserTeamRequest {
                user_id: user.user_id.to_string(),
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
                .get_user_team(grpc_req)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            let inner = resp.into_inner();
            if !inner.success {
                return Err(actix_web::error::ErrorForbidden("Team access denied"));
            }

            if inner.team_id != team_id {
                return Err(actix_web::error::ErrorForbidden("Team access denied"));
            }

            req.extensions_mut().insert(user);

            srv.call(req).await
        })
    }
}
