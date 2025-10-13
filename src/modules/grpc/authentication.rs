use crate::ledger::{
    GetUserTeamRequest, GetUserTeamResponse, ValidationRequest, ValidationResponse,
};
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

    pub async fn get_user_team(&self, user_id: &str) -> Result<GetUserTeamResponse, Status> {
        let mut client = self.authentication_client();
        let request = self.request_with_auth(GetUserTeamRequest {
            user_id: user_id.to_string(),
        });

        let response = client.get_user_team(request).await?;
        Ok(response.into_inner())
    }
}
