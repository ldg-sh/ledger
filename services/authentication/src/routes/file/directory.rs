use crate::middleware::middleware::AuthenticatedUser;
use actix_web::{post, web, HttpResponse};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::directory::{DirectoryRequest, DirectoryResponse};
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use storage::s3_manager::S3StorageManager;

#[post("create")]
pub async fn directory(
    database: web::Data<DatabaseConnection>,
    _s3_client: web::Data<S3StorageManager>,
    payload: web::Json<DirectoryRequest>,
    authenticated_user: AuthenticatedUser,
) -> HttpResponse {
    let directories = payload.name.split("/").collect::<Vec<&str>>();

    let mut inserts = Vec::new();
    let mut current_id = String::from("");
    let mut base_path = payload.path.clone();

    if directories.len() == 0 {
        return HttpResponse::BadRequest().finish();
    }

    if directories
        .iter()
        .any(|directory_name| directory_name.len() > 255)
    {
        return HttpResponse::BadRequest()
            .body("Directory names must be 255 characters or less".to_string());
    }

    directories.iter().for_each(|dir| {
        if !dir.is_empty() {
            let id = uuid::Uuid::new_v4().to_string();

            let insert = file::ActiveModel {
                id: Set(id.clone()),
                file_name: Set(dir.to_string()),
                owner_id: Set(authenticated_user.id.clone()),
                created_at: Set(DateTimeWithTimeZone::from(chrono::Utc::now())),
                upload_completed: Set(true),
                file_type: Set("directory".to_string()),
                file_size: Set(0),
                path: Set(base_path.clone()),
                is_directory: Set(true),
            };

            inserts.push(insert);

            base_path = dir.to_string();
            current_id = id;
        }
    });

    let insert = File::insert_many(inserts).exec(database.get_ref()).await;

    if insert.is_err() {
        return HttpResponse::InternalServerError().body(format!(
            "Failed to create file record: {}",
            insert.err().unwrap()
        ));
    }

    let response = DirectoryResponse {
        file_id: current_id.clone(),
    };

    HttpResponse::Ok().json(response)
}
