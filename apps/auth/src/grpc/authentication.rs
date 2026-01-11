use super::pb::{
    authentication_server::{Authentication, AuthenticationServer},
    ValidationRequest, ValidationResponse,
};
use crate::utils::token::{extract_token_parts, token_valid};
use crate::{config::config, db::postgres_service::PostgresService};
use std::sync::Arc;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct AuthenticationSvc {
    pub postgres_service: Arc<PostgresService>,
}

impl AuthenticationSvc {
    pub fn new(postgres_service: Arc<PostgresService>) -> Self {
        Self { postgres_service }
    }
}

#[tonic::async_trait]
impl Authentication for AuthenticationSvc {
    async fn validate_authentication(
        &self,
        request: Request<ValidationRequest>,
    ) -> Result<Response<ValidationResponse>, Status> {
        let header_token = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "None".to_string());

        let validation_request = request.into_inner();

        let body_token_valid = token_valid(&self.postgres_service, &validation_request.token).await;
        if header_token != config().grpc.auth_key {
            return Ok(Response::new(ValidationResponse {
                is_valid: false,
                user_id: "".to_string(),
                message: "Invalid authorization token.".into(),
            }));
        }

        let user_id = match extract_token_parts(&validation_request.token) {
            Some(user_id) => user_id.0,
            None => {
                return Ok(Response::new(ValidationResponse {
                    is_valid: false,
                    user_id: "".to_string(),
                    message: "Malformed token.".into(),
                }))
            }
        };

        Ok(Response::new(ValidationResponse {
            is_valid: body_token_valid,
            user_id: if body_token_valid {
                user_id.into()
            } else {
                "".into()
            },
            message: if body_token_valid {
                "ok".into()
            } else {
                "invalid".into()
            },
        }))
    }
}

pub fn server(postgres_service: Arc<PostgresService>) -> AuthenticationServer<AuthenticationSvc> {
    AuthenticationServer::new(AuthenticationSvc::new(postgres_service))
}
