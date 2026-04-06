use crate::middleware::middleware::AuthenticatedUser;
use actix_web::{HttpResponse, Responder, post, web};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::list::{Breadcrumb, ListFileElement, ListFilesRequest, ListFilesResponse};
use sea_orm::{ColumnTrait, Order, QueryOrder, QuerySelect};
use sea_orm::{ConnectionTrait, QueryFilter, Statement};
use sea_orm::{DatabaseConnection, EntityTrait};

#[post("list")]
pub async fn list(
    database: web::Data<DatabaseConnection>,
    payload: web::Json<ListFilesRequest>,
    authenticated_user: AuthenticatedUser,
) -> impl Responder {
    let limit = payload.limit.unwrap_or(20) as u64;
    let offset = payload.offset.unwrap_or(0) as u64;

    let (column, order) = match payload.sort.as_str() {
        "name_asc" => (file::Column::FileName, Order::Asc),
        "name_desc" => (file::Column::FileName, Order::Desc),
        "date_asc" => (file::Column::CreatedAt, Order::Asc),
        "date_desc" => (file::Column::CreatedAt, Order::Desc),
        "size_asc" => (file::Column::FileSize, Order::Asc),
        "size_desc" => (file::Column::FileSize, Order::Desc),
        "type_asc" => (file::Column::FileType, Order::Asc),
        "type_desc" => (file::Column::FileType, Order::Desc),
        _ => (file::Column::Id, Order::Desc),
    };
    
    println!("{:?}", payload);

    let files_query = File::find()
        .filter(file::Column::OwnerId.eq(authenticated_user.id.clone()))
        .filter(file::Column::Path.eq(payload.path.clone()))
        .order_by(file::Column::IsDirectory, Order::Desc)
        .order_by(column, order)
        .limit(limit + 1)
        .offset(offset)
        .all(database.get_ref());

    let path_clone = payload.path.clone();
    let database = database.clone();

    let breadcrumbs_future = async move {
        if path_clone.is_empty() || path_clone == "" {
            return Ok(vec![]);
        }

        let sql = r#"
            WITH RECURSIVE trail AS (
                SELECT id, file_name, path FROM file WHERE id = $1
                UNION ALL
                SELECT f.id, f.file_name, f.path FROM file f
                INNER JOIN trail t ON f.id = t.path
            )
            SELECT id, file_name FROM trail;
        "#;

        database.query_all_raw(Statement::from_sql_and_values(
            database.get_database_backend(),
            sql,
            [path_clone.into()],
        )).await
    };

    let (files_result, crumbs_result) = tokio::join!(files_query, breadcrumbs_future);

    let mut files = match files_result {
        Ok(list) => list,
        Err(e) => {
            log::error!("Error fetching files: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let has_more = files.len() as u64 > limit;
    if has_more { files.pop(); }

    let files_vec: Vec<_> = files
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

    let mut breadcrumbs = match crumbs_result {
        Ok(rows) => rows.into_iter().map(|row| Breadcrumb {
            id: row.try_get::<String>("", "id").unwrap_or_default(),
            name: row.try_get::<String>("", "file_name").unwrap_or_default(),
        }).collect::<Vec<_>>(),
        Err(e) => {
            log::error!("Breadcrumb error: {:?}", e);
            vec![]
        }
    };
    breadcrumbs.reverse();

    HttpResponse::Ok().json(ListFilesResponse {
        breadcrumbs,
        files: files_vec,
        has_more,
    })
}