use std::sync::Arc;
use actix_web::{delete, patch, post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::types::file::{TCreateDirectory, TCreateFile};
use crate::util::file::{build_key, is_directory};

#[derive(Serialize, Deserialize)]
struct RenameRequest {
    #[serde(rename = "newName")]
    pub new_name: String,
}

#[derive(Serialize, Deserialize)]
struct CopyRequest {
    #[serde(rename = "destinationPath")]
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

    if let Err(e) = postgres_service.create_directory(
        TCreateDirectory {
            id: id.clone(),
            file_name: last_segment,
            upload_id: String::new(),
            owner_id: authenticated_user.id.clone(),
            created_at: Utc::now(),
            path: without_last_segment.to_string(),
        }
    ).await {
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