use crate::middleware::middleware::AuthenticatedUser;
use actix_web::{post, web, HttpResponse};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::upload_complete::CompleteUploadRequest;
use common::types::file::upload_init::{InitUploadInternalRequest, InitUploadInternalResponse};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::sea_query::prelude::chrono;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use storage::s3_manager::S3StorageManager;
use storage::s3_scoped_storage::S3ScopedStorage;
use storage::StorageBackend;

#[post("init")]
pub async fn init(
    database: web::Data<DatabaseConnection>,
    payload: web::Json<InitUploadInternalRequest>,
    s3_scoped_storage: web::Data<S3StorageManager>,
    authenticated_user: AuthenticatedUser,
    _authenticated_user: AuthenticatedUser,
) -> HttpResponse {
    let insert = File::insert(file::ActiveModel {
        id: Set(payload.file_id.clone()),
        file_name: Set(payload.filename.clone()),
        owner_id: Set(payload.user_id.clone()),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        upload_completed: Set(false),
        file_type: Set(payload.content_type.clone()),
        file_size: Set(payload.size as i64),
        path: Set(payload.path.clone()),
        is_directory: Set(false),
    })
    .exec(database.get_ref())
    .await;

    if insert.is_err() {
        return HttpResponse::InternalServerError().body(format!(
            "Failed to create file record: {}",
            insert.err().unwrap()
        ));
    }

    let storage = S3ScopedStorage {
        user_id: authenticated_user.id.clone(),
        bucket: s3_scoped_storage.bucket.clone(),
        client: s3_scoped_storage.client.clone(),
    };

    let id = match storage.create_upload(&payload.file_id).await {
        Ok(res) => res,
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!(
                "Failed to create upload session: {}",
                err.to_string()
            ));
        }
    };

    HttpResponse::Ok().json(InitUploadInternalResponse { upload_id: id })
}

#[post("complete")]
pub async fn complete(
    database: web::Data<DatabaseConnection>,
    payload: web::Json<CompleteUploadRequest>,
    s3_scoped_storage: web::Data<S3StorageManager>,
    authenticated_user: AuthenticatedUser,
) -> HttpResponse {
    let storage = S3ScopedStorage {
        user_id: authenticated_user.id.clone(),
        bucket: s3_scoped_storage.bucket.clone(),
        client: s3_scoped_storage.client.clone(),
    };

    match storage
        .complete_upload(
            &payload.file_id,
            &payload.upload_id,
            payload
                .parts
                .iter()
                .map(|part| (part.part_number, part.etag.clone()))
                .collect::<Vec<(u32, String)>>(),
        )
        .await
    {
        Ok(_) => {}
        Err(err) => {
            if let Some(s3_err) = err.downcast_ref::<aws_sdk_s3::Error>() {
                println!("S3 Error: {:?}", s3_err);
            } else {
                println!("S3 Error: {:?}", err);
            }
            return HttpResponse::InternalServerError().body(format!(
                "S3 Error: {}", err
            ));
        }
    }

    let update = File::update_many()
        .col_expr(file::Column::UploadCompleted, true.into())
        .filter(file::Column::Id.eq(payload.file_id.clone()))
        .exec(database.get_ref())
        .await;

    println!("{:?}", update);

    if update.is_err() {
        HttpResponse::InternalServerError().body(format!(
            "Failed to update file record: {}",
            update.err().unwrap()
        ));
    }

    HttpResponse::Ok().finish()
}
