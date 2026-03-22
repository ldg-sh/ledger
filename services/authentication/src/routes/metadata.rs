use actix_web::{post, web, HttpResponse};
use common::entities::file;
use common::entities::prelude::File;
use common::types::metadata::{MetadataRequest, MetadataResponse};
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

#[post("metadata")]
pub async fn metadata(database: web::Data<DatabaseConnection>, payload: web::Json<MetadataRequest>) -> HttpResponse {
    let file = File::find()
        .filter(file::Column::Id.eq(payload.file_id.clone()))
        .one(database.get_ref())
        .await;

    if file.is_err() {
        return HttpResponse::InternalServerError().body(
            format!("Failed to create file record: {}", file.err().unwrap())
        )
    }

    let file = file.unwrap();

    if file.is_none() {
        return HttpResponse::NotFound().body(
            format!("File with ID {} not found", payload.file_id)
        )
    }

    match file {
        Some(data) => {
            HttpResponse::Ok().json(
                MetadataResponse {
                    filename: data.file_name,
                    size: data.file_size as u64,
                    content_type: data.file_type,
                    user_id: data.owner_id,
                    path: data.path,
                }
            )
        }
        None => {
            HttpResponse::NotFound().body(format!("File with ID {} not found", payload.file_id))
        }
    }
}
