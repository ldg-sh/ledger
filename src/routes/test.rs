use actix_web::{post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tonic::{transport::Channel, Request};

use crate::ledger::{authentication_client::AuthenticationClient, ValidationRequest};

#[post("")]
pub async fn test(
    bearer: BearerAuth,
    grpc: web::Data<Channel>
) -> impl Responder {
    let key = bearer.token();

    let mut client = AuthenticationClient::new(grpc.get_ref().clone());
    let req = Request::new(ValidationRequest {
            token: key.to_string(),
    });

    match client.validate_authentication(req).await {
        Ok(o) => {
            let res = o.into_inner();
            HttpResponse::Ok().body(res.message)
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        },
    }
}
