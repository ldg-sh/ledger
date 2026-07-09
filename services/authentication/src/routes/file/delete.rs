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
use crate::middleware::middleware::AuthenticatedUser;

#[delete("delete")]
pub async fn delete(
    database: web::Data<DatabaseConnection>,
    s3_manager: web::Data<S3StorageManager>,
    payload: web::Json<DeleteFilesRequest>,
    authenticated_user: AuthenticatedUser
) -> impl Responder {
    let file_ids = payload.into_inner().file_ids;

    let delete_result = File::delete_many()
        .filter(file::Column::Id.is_in(file_ids.clone()))
        .filter(file::Column::OwnerId.eq(authenticated_user.id.clone()))
        .exec(database.get_ref())
        .await;

    let result = match delete_result {
        Ok(res) => res,
        Err(err) => {
            log::error!("Database deletion failed: {}", err);
            return HttpResponse::InternalServerError().body("Failed to delete files.");
        }
    };

    if result.rows_affected == 0 {
        return HttpResponse::NotFound().finish();
    }

    let storage = S3ScopedStorage {
        user_id: authenticated_user.id.clone(),
        bucket: s3_manager.bucket.clone(),
        client: s3_manager.client.clone(),
    };

    tokio::spawn(
        async move {
            match storage.delete_many(file_ids.clone()).await {
                Ok(_) => log::info!("Successfully deleted files from S3: {:?}", file_ids),
                Err(err) => log::error!("Failed to delete files from S3: {}", err),
            }
        }
    );

    HttpResponse::Ok().finish()
}