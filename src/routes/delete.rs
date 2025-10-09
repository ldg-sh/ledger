extern crate sanitize_filename;

use crate::modules::s3::s3_service::S3Service;
use crate::util::strings::compound_team_file;
use actix_web::{HttpResponse, delete, web};
use std::sync::Arc;

#[delete("/delete")]
pub async fn delete(
    s3_service: web::Data<Arc<S3Service>>,
    team: web::Path<String>,
    key: web::Path<String>,
) -> HttpResponse {
    let compounded_name = compound_team_file(team.as_ref(), key.as_ref());

    match s3_service.delete(&compounded_name).await {
        Ok(m) => m,
        Err(e) => {
            log::error!("{e:?}");
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().finish()
}
