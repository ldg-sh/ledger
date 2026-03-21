use actix_web::{post, web, HttpResponse};
use common::api::upload_complete::CompleteUploadRequest;
use common::api::upload_init::{InitUploadRequest, InitUploadResponse};
use common::entities::file;
use common::entities::prelude::File;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::prelude::chrono;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use storage::s3_manager::S3StorageManager;
use storage::s3_scoped_storage::S3ScopedStorage;
use storage::StorageBackend;
use uuid::Uuid;

#[post("init")]
pub async fn init(database: web::Data<DatabaseConnection>, s3_client: web::Data<S3StorageManager>, payload: web::Json<InitUploadRequest>) -> HttpResponse {
    let storage = S3ScopedStorage {
        user_id: payload.user_id.clone(),
        bucket: s3_client.bucket.clone(),
        client: s3_client.client.clone()
    };

    let file_id = Uuid::new_v4().to_string();

    let presigned = storage.get_presigned_upload(&file_id).await;

    let insert = File::insert(
        file::ActiveModel {
            id: Set(file_id.clone()),
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

    match presigned {
        Ok(data) => {
            HttpResponse::Ok().json(
                InitUploadResponse {
                    file_id,
                    upload_url: data
                }
            )
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(
                format!("Failed to generate presigned URL: {}", err)
            )
        }
    }
}

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