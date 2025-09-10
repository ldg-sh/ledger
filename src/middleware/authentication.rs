use crate::config::config;
use crate::ledger::authentication_client::AuthenticationClient;
use crate::ledger::ValidationRequest;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::Request;

pub async fn validate_token(
    req: ServiceRequest,
    credentials: BearerAuth
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    println!("Hitting auth middleware");
    let grpc_client = &req
        .app_data::<actix_web::web::Data<tonic::transport::Channel>>()
        .ok_or_else(|| {
            (
                actix_web::error::ErrorInternalServerError("gRPC client not configured"),
                &req,
            )
        })
        .map(|data| data.get_ref().clone())
        .map_err(|_| actix_web::error::ErrorInternalServerError("gRPC client not configured"))
        .unwrap();

    let token = credentials.token().to_string();

    let mut client = AuthenticationClient::new(grpc_client.clone());
    let mut request = Request::new(ValidationRequest {
        token: token.to_string(),
    });

    let v: MetadataValue<Ascii> = match config().grpc.auth_key.parse() {
        Ok(v) => v,
        Err(e) => {
            return Err((actix_web::error::ErrorUnauthorized(e.to_string()), req))
        },
    };

    request.metadata_mut().insert("authorization", v);

    match client.validate_authentication(request).await {
        Ok(o) => {
            let r = o.into_inner();
            println!("Token validated res: {:?}", r);
            if !r.is_valid {
                println!("Token is far from valid wtf");
                return Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))
            }
        },
        Err(e) => {
            return Err((actix_web::error::ErrorInternalServerError(e.to_string()), req))
        },
    };

    Ok(req)
}
