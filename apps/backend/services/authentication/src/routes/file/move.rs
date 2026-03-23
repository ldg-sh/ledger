use actix_web::{post, web, HttpResponse, Responder};
use common::entities::file;
use common::entities::prelude::File;
use migration::Expr;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait};
use common::types::file::r#move::MoveFilesRequest;

#[post("move")]
pub async fn r#move(
    database: web::Data<DatabaseConnection>,
    payload: web::Json<MoveFilesRequest>,
) -> impl Responder {
    match File::update_many()
        .col_expr(
            file::Column::Path,
            Expr::Value(payload.destination_path.clone().into()),
        )
        .filter(file::Column::Id.is_in(payload.file_ids.clone()))
        .exec(database.get_ref())
        .await
    {
        Ok(_) => {}
        Err(err) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to update the files: {:?}", err));
        }
    }

    HttpResponse::Ok().finish()
}
