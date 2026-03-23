use actix_web::{post, web, HttpResponse};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::directory::DirectoryRequest;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use storage::s3_manager::S3StorageManager;

#[post("directory")]
pub async fn directory(database: web::Data<DatabaseConnection>, _s3_client: web::Data<S3StorageManager>, payload: web::Json<DirectoryRequest>) -> HttpResponse {
    let insert = File::insert(
        file::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            file_name: Set(payload.name.clone()),
            owner_id: Set(payload.user_id.clone()),
            created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
            upload_completed: Set(true),
            file_type: Set("directory".to_string()),
            file_size: Set(0),
            path: Set(payload.path.clone())
        }
    ).exec(database.get_ref()).await;

    if insert.is_err() {
        return HttpResponse::InternalServerError().body(
            format!("Failed to create file record: {}", insert.err().unwrap())
        )
    }

    HttpResponse::Ok().finish()
}