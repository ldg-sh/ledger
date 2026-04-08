use actix_web::{Error, HttpResponse, post, web};
use common::types::file::explode::{ExplodeRequest, ExplodedItem};
use sea_orm::{DatabaseConnection, DbBackend, FromQueryResult, Statement};

#[post("/explode")]
pub async fn explode(
    db: web::Data<DatabaseConnection>,
    req: web::Json<ExplodeRequest>,
) -> Result<HttpResponse, Error> {
    let db = db.get_ref();

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

    let exploded_items: Vec<ExplodedItem> = ExplodedItem::find_by_statement(
        Statement::from_sql_and_values(DbBackend::Postgres, query, [req.item_ids.clone().into()]),
    )
    .all(db)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(exploded_items))
}
