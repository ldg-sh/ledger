use crate::{
    modules::s3::s3_service::S3Service,
    types::response::{ApiResponse, ApiResult},
};
use actix_web::{HttpResponse, post, web};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Response {
    message: String,
}

#[post("/{id}")]
pub async fn setup(
    s3_service: web::Data<Arc<S3Service>>,
    path: web::Path<Uuid>,
) -> ApiResult<Response> {
    s3_service
        .create_team_folder(&path.into_inner().to_string())
        .await?;

    Ok(ApiResponse::Ok(Response {
        message: "Ok".to_string(),
    }))
}
