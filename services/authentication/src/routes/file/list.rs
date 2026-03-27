use actix_web::{HttpResponse, Responder, post, web};
use common::entities::file;
use common::entities::prelude::File;
use sea_orm::{ColumnTrait, Order, QueryOrder, QuerySelect};
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
    let (column, order) = match payload.sort.as_str() {
        "name_asc" => (file::Column::FileName, Order::Asc),
        "name_desc" => (file::Column::FileName, Order::Desc),
        "date_asc" => (file::Column::CreatedAt, Order::Asc),
        "date_desc" => (file::Column::CreatedAt, Order::Desc),
        "size_asc" => (file::Column::FileSize, Order::Asc),
        "size_desc" => (file::Column::FileSize, Order::Desc),
        _ => (file::Column::Id, Order::Desc),
    };

    let files = match File::find()
        .filter(file::Column::OwnerId.eq(authenticated_user.id.clone()))
        .filter(file::Column::Path.eq(payload.path.clone()))
        .order_by(column, order)
        .limit(payload.limit.unwrap_or(100) as u64)
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
