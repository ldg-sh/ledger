use actix_web::{post, web, HttpResponse};
use common::entities::file;
use common::entities::prelude::File;
use common::types::upload_complete::CompleteUploadRequest;
use common::types::upload_init::InitUploadInternalRequest;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::prelude::chrono;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use storage::s3_manager::S3StorageManager;

#[post("init")]
pub async fn init(database: web::Data<DatabaseConnection>, _s3_client: web::Data<S3StorageManager>, payload: web::Json<InitUploadInternalRequest>) -> HttpResponse {
    let insert = File::insert(
        file::ActiveModel {
            id: Set(payload.file_id.clone()),
            file_name: Set(payload.filename.clone()),
            owner_id: Set(payload.user_id.clone()),
            created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
            upload_completed: Set(false),
            file_type: Set(payload.content_type.clone()),
            file_size: Set(payload.size as i64),
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

#[post("complete")]
pub async fn complete(database: web::Data<DatabaseConnection>, payload: web::Json<CompleteUploadRequest>) -> HttpResponse {
    let update = File::update_many()
        .col_expr(file::Column::UploadCompleted, true.into())
        .filter(file::Column::Id.eq(payload.file_id.clone()))
        .exec(database.get_ref())
        .await;

    if update.is_err() {
        HttpResponse::InternalServerError().body(
            format!("Failed to update file record: {}", update.err().unwrap())
        );
    }

    HttpResponse::Ok().finish()
}