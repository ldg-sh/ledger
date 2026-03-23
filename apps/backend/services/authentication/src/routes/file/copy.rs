use actix_web::{post, web, HttpResponse, Responder};
use common::entities::file;
use common::entities::prelude::File;
use futures::stream;
use futures::StreamExt;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use common::types::file::copy::CopyFilesRequest;
use storage::s3_manager::S3StorageManager;
use storage::s3_scoped_storage::S3ScopedStorage;
use storage::StorageBackend;

#[post("copy")]
pub async fn copy(
    database: web::Data<DatabaseConnection>,
    s3storage_manager: web::Data<S3StorageManager>,
    payload: web::Json<CopyFilesRequest>,
) -> impl Responder {
    let files = match File::find()
        .filter(file::Column::Id.is_in(payload.file_ids.clone()))
        .all(database.get_ref())
        .await
    {
        Ok(files) => files,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to fetch the files: {:?}", err));
        }
    };

    let s3_manager = S3ScopedStorage {
        user_id: payload.user_id.clone(),
        bucket: s3storage_manager.bucket.clone(),
        client: s3storage_manager.client.clone(),
    };

    let results: Vec<_> = stream::iter(files)
        .map(|file| {
            let s3_manager = s3_manager.clone();
            let payload = payload.clone();
            
            async move {
                s3_manager.copy_object(&file.path, &payload.destination_path).await
            }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    for file in results {
        match file {
            Ok(_) => {}
            Err(err) => {
                return HttpResponse::InternalServerError().body(format!("Failed to copy one or more files: {:?}", err));
            }
        }
    }
    
    HttpResponse::Ok().finish()
}
