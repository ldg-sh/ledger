use actix_web::{HttpResponse, post, web};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::directory::{DirectoryRequest, DirectoryResponse};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use storage::s3_manager::S3StorageManager;
use crate::middleware::middleware::AuthenticatedUser;

#[post("create")]
pub async fn directory(
    database: web::Data<DatabaseConnection>,
    _s3_client: web::Data<S3StorageManager>,
    payload: web::Json<DirectoryRequest>,
    authenticated_user: AuthenticatedUser
) -> HttpResponse {
    let id = uuid::Uuid::new_v4().to_string();
    let insert = File::insert(file::ActiveModel {
        id: Set(id.clone()),
        file_name: Set(payload.name.clone()),
        owner_id: Set(authenticated_user.id.clone()),
        created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
        upload_completed: Set(true),
        file_type: Set("directory".to_string()),
        file_size: Set(0),
        path: Set(payload.path.clone()),
    })
    .exec(database.get_ref())
    .await;

    if insert.is_err() {
        return HttpResponse::InternalServerError().body(format!(
            "Failed to create file record: {}",
            insert.err().unwrap()
        ));
    }

    let response = DirectoryResponse {
        file_id: id.clone(),
    };

    HttpResponse::Ok().json(response)
}
