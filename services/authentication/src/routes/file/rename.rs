use actix_web::{post, web, HttpResponse, Responder};
use common::entities::file;
use common::entities::prelude::File;
use migration::Expr;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use common::types::file::rename::RenameFileRequest;

#[post("rename")]
pub async fn rename(
    database: web::Data<DatabaseConnection>,
    payload: web::Json<RenameFileRequest>,
) -> impl Responder {
    match File::update_many()
        .col_expr(
            file::Column::FileName,
            Expr::Value(payload.file_name.to_owned().into()),
        )
        .filter(file::Column::Id.eq(payload.file_name.to_owned()))
        .exec(database.get_ref())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to rename the file: {:?}", err));
        }
    }

    HttpResponse::Ok().finish()
}
