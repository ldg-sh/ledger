use std::sync::Arc;
use actix_web::{delete, patch, post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::types::file::TCreateFile;
use crate::util::file::{build_key};

#[derive(Serialize, Deserialize)]
struct RenameRequest {
    pub new_name: String,
}

#[derive(Serialize, Deserialize)]
struct CopyRequest {
    pub destination_path: String,
}

#[post("/create/{path:.*}")]
pub async fn create(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    let postgres_service = &context.postgres_service;
    let path = path.into_inner();

    let last_segment = path
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string();

    let without_last_segment = if let Some(pos) = path.rfind('/') {
        &path[..pos]
    } else {
        ""
    };

    let id = uuid::Uuid::new_v4().to_string();

    let new_file = TCreateFile {
        id: id.clone(),
        file_name: last_segment,
        upload_id: "".to_string(),
        owner_id: authenticated_user.id.clone(),
        file_size: 0,
        created_at: Utc::now(),
        file_type: "directory".to_string(),
        upload_completed: true,
        path: without_last_segment.to_string(),
    };

    if let Err(e) = postgres_service.create_file(new_file).await {
        log::error!("DB create directory failed: {:?}", e);

        return HttpResponse::InternalServerError().body(
            "Failed to create directory in database."
        )
    }

    HttpResponse::Ok().json(json!({"folder_id": id}))
}

#[delete("/{path:.*}")]
pub async fn delete(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    let path = path.into_inner();
    let db = &context.postgres_service;
    let s3 = &context.s3_service;

    let files = match db.list_related_files(&path, &authenticated_user.id).await {
        Ok(f) => f,
        Err(e) => {
            log::error!("DB list related files failed: {:?}", e);

            return HttpResponse::InternalServerError().body(
                "Failed to list related files in database."
            )
        }
    };

    if let Err(e) = db.delete_prefix(&path, &authenticated_user.id).await {
        log::error!("DB prefix delete failed: {:?}", e);

        return HttpResponse::InternalServerError().body(
            "Failed to delete directory in database."
        )
    }

    let keys: Vec<String> = files.iter().map(|f| {
        build_key(
            &authenticated_user,
            f.id.as_str(),
        )
    }).collect();

    if let Err(e) = s3.delete_multiple_files(keys).await {
        log::error!("S3 multiple delete failed: {:?}", e);

        return HttpResponse::InternalServerError().body(
            "Failed to delete directory in storage."
        )
    }

    HttpResponse::Ok().finish()
}

#[patch("/{path:.*}")]
pub async fn rename(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    path: web::Path<String>,
    rename_request: web::Json<RenameRequest>,
) -> HttpResponse {
    let old_path = path.into_inner();
    let new_path = rename_request.new_name.clone();

    if let Err(e) = context.postgres_service.move_prefix(&old_path, &new_path, &authenticated_user.id).await {
        log::error!("DB Move failed: {:?}", e);
        return HttpResponse::InternalServerError().body("DB update failed");
    }

    HttpResponse::Ok().finish()
}

#[post("/copy/{path:.*}")]
pub async fn copy(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    path: web::Path<String>,
    copy_request: web::Json<CopyRequest>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let files = postgres_service.list_related_files(&path, &authenticated_user.id).await;

    if files.is_err() {
        log::error!("Failed to list files for copy: {:?}", files.unwrap_err());

        return HttpResponse::InternalServerError().body(
            "Failed to list files for copy."
        );
    }

    let files = files.unwrap();
    let mut new_db_entries = Vec::new();

    for file in files.clone() {
        let new_id = uuid::Uuid::new_v4().to_string();

        new_db_entries.push(TCreateFile {
            id: new_id,
            file_name: file.file_name,
            upload_id: file.upload_id,
            owner_id: file.owner_id,
            file_size: file.file_size,
            created_at: Utc::now(),
            file_type: file.file_type,
            upload_completed: file.upload_completed,
            path: copy_request.destination_path.clone(),
        })
    }

    let source_keys: Vec<String> = files.iter().map(|f| {
        build_key(
            &authenticated_user,
            f.id.as_str(),
        )
    }).collect();

    let destination_keys: Vec<String> = new_db_entries.iter().map(|f| {
        build_key(
            &authenticated_user,
            f.id.as_str(),
        )
    }).collect();

    let mut s3_tasks = Vec::new();
    for (src, dest) in source_keys.iter().zip(destination_keys.iter()) {
        let s3_clone = Arc::clone(&s3_service);
        let src_clone = src.clone();
        let dest_clone = dest.clone();

        s3_tasks.push(async move {
            s3_clone.copy_file(&src_clone, &dest_clone).await
        });
    }

    if let Err(e) = futures::future::join_all(s3_tasks).await.into_iter().collect::<Result<Vec<_>, _>>() {
        log::error!("S3 Copy Failed: {:?}", e);

        return HttpResponse::InternalServerError().body("S3 Copy Failed");
    }

    if let Err(e) = postgres_service.create_multiple(new_db_entries).await {
        log::error!("DB Batch Insert Failed: {:?}", e);

        return HttpResponse::InternalServerError().body("DB Batch Insert Failed");
    }

    HttpResponse::Ok().finish()
}