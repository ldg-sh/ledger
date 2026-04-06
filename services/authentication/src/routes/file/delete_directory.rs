use crate::middleware::middleware::AuthenticatedUser;
use actix_web::{delete, web, HttpResponse, Responder};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::directory_delete::DeleteDirectoryRequest;
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, Statement};
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
    let backend = database.get_database_backend();
    let sql = r#"
        WITH RECURSIVE subordinates AS (
            SELECT id, is_directory FROM file WHERE id = $1 AND owner_id = $2
            UNION ALL
            SELECT f.id, f.is_directory FROM file f
            INNER JOIN subordinates s ON f.path = s.id
            WHERE f.owner_id = $2
        )
        SELECT id, is_directory FROM subordinates;
    "#;

    let rows = match database
        .query_all_raw(Statement::from_sql_and_values(
            backend,
            sql,
            [payload.directory_id.clone().into(), authenticated_user.id.clone().into()],
        ))
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            log::error!("Recursive fetch error: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut all_ids = Vec::new();
    let mut file_ids_for_s3 = Vec::new();

    for row in rows {
        let id = row.try_get::<String>("", "id").unwrap_or_default();
        let is_dir = row.try_get::<bool>("", "is_directory").unwrap_or_default();

        all_ids.push(id.clone());
        if !is_dir {
            file_ids_for_s3.push(id);
        }
    }

    if all_ids.is_empty() {
        return HttpResponse::NotFound().finish();
    }

    let storage = S3ScopedStorage {
        user_id: authenticated_user.id.clone(),
        bucket: s3_manager.bucket.clone(),
        client: s3_manager.client.clone(),
    };

    if !file_ids_for_s3.is_empty() {
        if let Err(e) = storage.delete_many(file_ids_for_s3).await {
            log::error!("S3 delete error: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to sync with storage");
        }
    }

    match File::delete_many()
        .filter(file::Column::Id.is_in(all_ids))
        .exec(database.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Database delete error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}