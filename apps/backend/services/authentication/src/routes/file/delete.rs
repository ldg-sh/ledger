use actix_web::{HttpResponse, Responder, delete, web};
use common::entities::file;
use common::entities::prelude::File;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait};
use common::types::file::delete::DeleteFilesRequest;
use storage::s3_manager::S3StorageManager;
use storage::s3_scoped_storage::S3ScopedStorage;
use storage::StorageBackend;

#[delete("delete")]
pub async fn delete(
    database: web::Data<DatabaseConnection>,
    s3_manager: web::Data<S3StorageManager>,
    payload: web::Json<DeleteFilesRequest>,
) -> impl Responder {
    let result = match File::delete_many()
        .filter(file::Column::Id.is_in(payload.file_ids.clone()))
        .exec(database.get_ref())
        .await
        .map_err(|err| {
            HttpResponse::InternalServerError().body(format!("Failed to delete files: {}", err));
        }) {
        Ok(result) => result,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to delete files.");
        }
    };

    if result.rows_affected == 0 {
        return HttpResponse::NotFound().finish();
    }

    let storage = S3ScopedStorage {
        user_id: payload.user_id.clone(),
        bucket: s3_manager.bucket.clone(),
        client: s3_manager.client.clone(),
    };

    match storage.delete_many(
        payload.file_ids.clone()
    ).await {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Failed to delete files from S3: {}", err));
        }
    }

    HttpResponse::Ok().finish()
}
