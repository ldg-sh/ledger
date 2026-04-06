use actix_web::{HttpResponse, Responder, post, web};
use common::entities::file;
use common::entities::prelude::File;
use common::types::file::rename::RenameFileRequest;
use migration::Expr;
use sea_orm::{ExprTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
#[post("rename")]
pub async fn rename(
    database: web::Data<DatabaseConnection>,
    payload: web::Json<RenameFileRequest>,
) -> impl Responder {
    let db = database.get_ref();

    let file_to_rename = match File::find_by_id(payload.file_id.to_owned()).one(db).await {
        Ok(Some(f)) => f,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let old_name = file_to_rename.file_name.clone();
    let new_name = payload.file_name.clone();
    let parent_path = file_to_rename.path.clone();

    let old_path_value = if parent_path.is_empty() {
        old_name.clone()
    } else {
        format!("{}/{}", parent_path, old_name)
    };

    let new_path_value = if parent_path.is_empty() {
        new_name.clone()
    } else {
        format!("{}/{}", parent_path, new_name)
    };

    if file_to_rename.file_type == "directory" {
        let update_children = File::update_many()
            .filter(
                file::Column::Path
                    .eq(old_path_value.clone())
                    .or(file::Column::Path.like(format!("{}/%", old_path_value))),
            )
            .col_expr(
                file::Column::Path,
                Expr::cust_with_exprs(
                    "CASE
                        WHEN path = $1 THEN $2
                        ELSE REPLACE(path, $3, $4)
                    END",
                    [
                        old_path_value.clone().into(),
                        new_path_value.clone().into(),
                        format!("{}/", old_path_value).into(),
                        format!("{}/", new_path_value).into(),
                    ],
                ),
            )
            .exec(db)
            .await;

        if let Err(e) = update_children {
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    }

    let update_self = File::update_many()
        .filter(file::Column::Id.eq(payload.file_id.to_owned()))
        .col_expr(file::Column::FileName, Expr::value(new_name))
        .exec(db)
        .await;

    match update_self {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
