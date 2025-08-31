use actix_web::{post, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tonic::{metadata::{Ascii, MetadataValue}, transport::Channel, Request};

use crate::{config::config, ledger::{authentication_client::AuthenticationClient, ValidationRequest}};

#[post("")]
pub async fn test(
    bearer: BearerAuth,
    grpc: web::Data<Channel>
) -> impl Responder {
    let key = bearer.token();

    let mut client = AuthenticationClient::new(grpc.get_ref().clone());
    let mut req = Request::new(ValidationRequest {
            token: key.to_string(),
    });

    let v: MetadataValue<Ascii> = match config().grpc.auth_key.parse() {
        Ok(v) => v,
        Err(e) => {
            return HttpResponse::InternalServerError().body(e.to_string())
        },
    };

    req.metadata_mut().insert("authorization", v);

    match client.validate_authentication(req).await {
        Ok(o) => {
            let res = o.into_inner();
            if res.is_valid {
                return HttpResponse::Ok().body(res.message)
            }
            HttpResponse::Unauthorized().body(res.message)
        },
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        },
    }
}
