use actix_web::get;
use serde::{Deserialize, Serialize};

use crate::types::response::{ApiResponse, ApiResult};

#[derive(Serialize, Deserialize)]
pub struct Response {
    message: String,
}

#[get("")]
async fn health(_req: actix_web::HttpRequest) -> ApiResult<Response> {
    Ok(ApiResponse::Ok(Response {
        message: "ok".to_string(),
    }))
}
