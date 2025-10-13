use crate::modules::grpc::grpc_service::GrpcService;
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;
use std::sync::Arc;

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

        let grpc_service = match req.app_data::<actix_web::web::Data<Arc<GrpcService>>>() {
            Some(c) => Arc::clone(c.get_ref()),
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

            let resp = grpc_service
                .get_user_team(&user.user_id)
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))?;

            if !resp.success {
                return Err(actix_web::error::ErrorForbidden("Team access denied"));
            }

            if resp.team_id != team_id {
                return Err(actix_web::error::ErrorForbidden("Team access denied"));
            }

            req.extensions_mut().insert(user);

            srv.call(req).await
        })
    }
}
