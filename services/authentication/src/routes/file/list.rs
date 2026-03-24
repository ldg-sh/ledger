use actix_web::{HttpResponse, Responder, post, web};
use common::entities::file;
use common::entities::prelude::File;
use sea_orm::ColumnTrait;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter};
use common::types::file::list::{ListFileElement, ListFilesRequest, ListFilesResponse};
use crate::middleware::middleware::AuthenticatedUser;

#[post("list")]
pub async fn list(
    database: web::Data<DatabaseConnection>,
    payload: web::Json<ListFilesRequest>,
    authenticated_user: AuthenticatedUser,
) -> impl Responder {
    let files = match File::find()
        .filter(file::Column::OwnerId.eq(authenticated_user.id.clone()))
        .filter(file::Column::Path.eq(payload.path.clone()))
        .all(database.get_ref())
        .await
    {
        Ok(list) => list,
        Err(e) => {
            log::error!("Error fetching files: {:?}", e);
            return HttpResponse::InternalServerError().body("Failed to fetch files");
        }
    };

    let files: Vec<_> = files
        .into_iter()
        .map(|v| ListFileElement {
            id: v.id,
            file_name: v.file_name,
            file_size: v.file_size,
            file_type: v.file_type,
            created_at: v.created_at,
            path: v.path,
            upload_completed: v.upload_completed,
        })
        .collect();

    let cleaned = ListFilesResponse { files };

    HttpResponse::Ok().json(cleaned)
}
