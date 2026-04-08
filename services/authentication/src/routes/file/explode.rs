use crate::middleware::middleware::AuthenticatedUser;
use actix_web::{HttpResponse, post, web};
use aws_sdk_s3::presigning::PresigningConfig;
use common::types::file::explode::{ZipRequest, ExplodeResponse, ExplodedItem, PresignedExplodedItem};
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};
use storage::s3_manager::S3StorageManager;
use storage::s3_scoped_storage::S3ScopedStorage;

#[post("/explode")]
pub async fn explode(
    database: web::Data<DatabaseConnection>,
    s3_client: web::Data<S3StorageManager>,
    req: web::Json<ZipRequest>,
    authenticated_user: AuthenticatedUser,
) -> HttpResponse {
    let database = database.get_ref();

    // language=PostgreSQL
    let query = r#"
    WITH RECURSIVE tree AS (
        SELECT
            id,
            file_name,
            is_directory,
            path,
            file_name::text as virtual_path
        FROM "file"
        WHERE id = ANY($1)

        UNION ALL

        SELECT
            i.id,
            i.file_name,
            i.is_directory,
            i.path,
            t.virtual_path || '/' || i.file_name
        FROM "file" i
        INNER JOIN tree t ON i.path = t.id
    )
    SELECT id, is_directory, file_name, virtual_path FROM tree WHERE is_directory = false;
"#;

    let exploded_items: Vec<ExplodedItem> = match ExplodedItem::find_by_statement(
        Statement::from_sql_and_values(DbBackend::Postgres, query, [req.item_ids.clone().into()]),
    )
    .all(database)
    .await
    {
        Ok(items) => items,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to execute query: {}", err.to_string()));
        }
    };

    let storage = S3ScopedStorage {
        user_id: authenticated_user.id.clone(),
        bucket: s3_client.bucket.clone(),
        client: s3_client.client.clone(),
    };

    let mut presigned_urls = Vec::new();

    let presign_config = PresigningConfig::builder()
        .expires_in(std::time::Duration::from_mins(30))
        .build()
        .unwrap();

    for item in exploded_items.clone() {
        let res = storage
            .client
            .get_object()
            .bucket(s3_client.bucket.clone())
            .key(format!("{}/{}", authenticated_user.id, item.id))
            .presigned(presign_config.clone())
            .await;

        if res.is_err() {
            return HttpResponse::InternalServerError().finish();
        }

        presigned_urls.push(PresignedExplodedItem {
            id: item.id.clone(),
            file_name: item.file_name.clone(),
            virtual_path: item.virtual_path.clone(),
            presign_url: res.unwrap().uri().to_string(),
        });
    }

    let explode_response = ExplodeResponse {
        items: presigned_urls,
    };

    HttpResponse::Ok().json(explode_response)
}
