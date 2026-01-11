use actix_web::{post, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::postgres_service::PostgresService;
use crate::types::error::AppError;
use crate::types::response::{ApiResponse, ApiResult};
use crate::utils::token::token_valid;

#[derive(Serialize, Deserialize)]
pub struct Response {}

#[post("")]
async fn validate(
    _req: actix_web::HttpRequest,
    auth: BearerAuth,
    db: web::Data<Arc<PostgresService>>,
) -> ApiResult<Response> {
    if !token_valid(&db, auth.token()).await {
        return Err(AppError::Unauthorized);
    }

    Ok(ApiResponse::EmptyOk)
}
