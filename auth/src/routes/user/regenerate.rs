use std::sync::Arc;

use actix_web::{post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::types::error::AppError;
use crate::types::response::{ApiResponse, ApiResult};
use crate::{
    db::postgres_service::PostgresService,
    types::mail::SendEmail,
    utils::{
        mail::send_email,
        token::{construct_token, extract_token_parts},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}

#[post("")]
async fn regenerate(
    _req: actix_web::HttpRequest,
    db: web::Data<Arc<PostgresService>>,
    auth: BearerAuth,
) -> ApiResult<Response> {
    let user_id = match extract_token_parts(auth.token()) {
        Some(user_id) => user_id.0,
        None => return Err(AppError::Unauthorized),
    };

    let new_token = db.regenerate_user_token(&user_id).await?;

    let user_email = db.get_user_by_id(&user_id).await?.email;

    let key = construct_token(&user_id, &new_token);

    let _ = send_email(SendEmail {
        from: "me@mail.noahdunnagan.com".to_string(),
        to: vec![user_email],
        subject: "Ledger access token reset.".to_string(),
        text: Some(format!("Your ledger access token has been reset. If this wasn't you, please contact support. \n \nYour new access key is: {}", key)),
        ..Default::default()
    }).await;

    Ok(ApiResponse::Ok(Response {
        message: "Regenerated user token, email has been sent with updated token.".to_string(),
    }))
}
