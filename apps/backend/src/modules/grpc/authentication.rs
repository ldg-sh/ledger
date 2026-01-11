use crate::ledger::{ValidationRequest, ValidationResponse};
use crate::modules::grpc::grpc_service::GrpcService;
use tonic::Status;

impl GrpcService {
    pub async fn validate_authentication(&self, token: &str) -> Result<ValidationResponse, Status> {
        let mut client = self.authentication_client();
        let request = self.request_with_auth(ValidationRequest {
            token: token.to_string(),
        });

        let response = client.validate_authentication(request).await?;
        Ok(response.into_inner())
    }
}
