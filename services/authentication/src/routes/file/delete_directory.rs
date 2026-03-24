use crate::middleware::middleware::AuthenticatedUser;
use actix_web::{delete, web, HttpResponse, Responder};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::directory_delete::DeleteDirectoryRequest;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use storage::s3_manager::S3StorageManager;
use storage::s3_scoped_storage::S3ScopedStorage;
use storage::StorageBackend;

#[delete("delete")]
pub async fn delete(
    database: web::Data<DatabaseConnection>,
    s3_manager: web::Data<S3StorageManager>,
    payload: web::Json<DeleteDirectoryRequest>,
    authenticated_user: AuthenticatedUser,
) -> impl Responder {
    let files_to_delete = File::find()
        .filter(file::Column::Path.starts_with(payload.path.clone()))
        .all(database.get_ref())
        .await
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()));

    let files_to_delete = match files_to_delete {
        Ok(files) => {
            if files.is_empty() {
                return HttpResponse::NotFound().body("File list not found");
            }

            files
        }
        Err(err) => {
            return err
        }
    };

    let delete_result = File::delete_many()
        .filter(file::Column::Path.starts_with(payload.path.clone()))
        .exec(database.get_ref())
        .await
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()));

    let delete_result = match delete_result {
        Ok(delete_result) => delete_result,
        Err(err) => {
            return err
        }
    };

    if delete_result.rows_affected == 0 {
        return HttpResponse::NotFound().finish();
    }

    let storage = S3ScopedStorage {
        user_id: authenticated_user.id.clone(),
        bucket: s3_manager.bucket.clone(),
        client: s3_manager.client.clone(),
    };

    match storage.delete_many(
        files_to_delete.iter().map(|f| f.id.clone()).collect(),
    ).await {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError().body(format!("Failed to delete files from S3: {}", err));
        }
    }

    HttpResponse::Ok().finish()
}
