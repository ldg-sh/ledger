use std::sync::Arc;
use actix_web::{delete, post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::util::file::{build_key};

#[derive(Serialize, Deserialize)]
pub struct CopyRequest {
    #[serde(rename = "fileIds")]
    pub file_ids: Vec<String>,
    #[serde(rename = "destinationPath")]
    pub destination_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteRequest {
    #[serde(rename = "fileIds")]
    pub file_ids: Vec<String>,
}
use futures::future::join_all;
use crate::types::file::TCreateFile;

#[post("/copy")]
pub async fn copy(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    copy_request: web::Json<CopyRequest>,
) -> HttpResponse {
    let s3 = &context.s3_service;
    let db = &context.postgres_service;

    let files = match db.get_multiple_files(
        copy_request.file_ids.iter().map(|id| id.as_str()).collect(),
        &authenticated_user.id
    ).await {
        Ok(f) => f,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let mut new_db_entries = Vec::new();
    let mut s3_tasks = Vec::new();

    for file in files {
        let new_id = uuid::Uuid::new_v4().to_string();
        let src = build_key(&authenticated_user, &file.id);
        let new_key = build_key(&authenticated_user, &new_id);

        new_db_entries.push(TCreateFile {
            id: new_id,
            file_name: file.file_name,
            upload_id: file.upload_id,
            owner_id: file.owner_id,
            file_size: file.file_size,
            created_at: Utc::now(),
            upload_completed: file.upload_completed,
            file_type: file.file_type,
            path: copy_request.destination_path.clone(),
        });

        s3_tasks.push(async move { s3.copy_file(&src, &new_key).await });
    }

    if let Err(_) = join_all(s3_tasks).await.into_iter().collect::<Result<Vec<_>, _>>() {
        return HttpResponse::InternalServerError().body("S3 Copy Failed");
    }

    if let Err(_) = db.create_multiple(new_db_entries).await {
        return HttpResponse::InternalServerError().body("DB Batch Insert Failed");
    }

    HttpResponse::Ok().finish()
}

#[delete("")]
pub async fn delete(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    delete_request: web::Json<DeleteRequest>,
) -> HttpResponse {
    let s3 = &context.s3_service;
    let db = &context.postgres_service;

    let file_ids: Vec<String> = delete_request.file_ids.clone();

    let keys: Vec<String> = file_ids
        .clone()
        .into_iter().map(|f| build_key(&authenticated_user, &f))
        .collect();

    if let Err(_) = s3.delete_multiple_files(keys).await {
        return HttpResponse::InternalServerError().finish();
    }

    if let Err(_) = db.delete_multiple(file_ids, &authenticated_user.id).await {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[post("/move")]
pub async fn r#move(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    move_request: web::Json<CopyRequest>,
) -> HttpResponse {
    let db = &context.postgres_service;

    let ids: Vec<String> = move_request.file_ids.clone();
    if let Err(_) = db.move_multiple(ids, &move_request.destination_path, &authenticated_user.id).await {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}