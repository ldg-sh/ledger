use std::sync::Arc;
use actix_web::{delete, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::util::file::{build_key, build_key_from_path};

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

#[post("/copy")]
pub async fn copy(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    copy_request: web::Json<CopyRequest>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let files = postgres_service.get_multiple_files(
        copy_request.file_ids.iter().map(
            |id| id.as_str()
        ).collect::<Vec<&str>>(),
        &authenticated_user.id
    ).await;

    if files.is_err() {
        return HttpResponse::InternalServerError().body(
            "Failed to retrieve files for copying."
        )
    }

    let files = files.unwrap();
    let mut new_file_ids = Vec::new();

    for file in files {
        let original_path = file.path.clone();
        let new_file_id = uuid::Uuid::new_v4().to_string();
        new_file_ids.push(new_file_id.clone());

        let copy = postgres_service.copy_file(
            file.clone(),
            &new_file_id,
            &copy_request.destination_path,
        ).await;

        if copy.is_err() {
            return HttpResponse::InternalServerError().body(
                "Failed to copy file in database."
            )
        }

        let source_key = build_key_from_path(
            &authenticated_user,
            &format!("{}/{}", original_path, file.id)
        );

        let destination_key = build_key(
            &authenticated_user,
            Some(copy_request.destination_path.as_str()),
            &new_file_id,
        );

        let s3_copy = s3_service.copy_file(
            &source_key,
            &destination_key,
        ).await;

        if s3_copy.is_err() {
            return HttpResponse::InternalServerError().body(
                "Failed to copy file in storage."
            )
        }
    }

    HttpResponse::Ok().json(
        serde_json::json!({
            "file_ids": new_file_ids,
        })
    )
}

#[delete("")]
pub async fn delete(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    delete_request: web::Json<DeleteRequest>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.clone().into_inner().postgres_service);

    let files = postgres_service.get_multiple_files(
        delete_request.file_ids.iter().map(
            |id| id.as_str()
        ).collect::<Vec<&str>>(),
        &authenticated_user.id
    ).await;

    if files.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let files = files.unwrap();

    let delete_res = s3_service.delete_multiple_files(
        files.iter().map(|file| {
            build_key_from_path(
                &authenticated_user,
                &file.path,
            )
        }).collect()).await;

    if delete_res.is_err() {
        return HttpResponse::InternalServerError().body("Failed to delete files from storage.");
    }

    let db_delete_res = postgres_service.delete_multiple(
        files
    ).await;

    if db_delete_res.is_err() {
        return HttpResponse::InternalServerError().body("Failed to delete files from database.");
    }

    HttpResponse::Ok().finish()
}

#[post("/move")]
pub async fn r#move(
    context: web::Data<Arc<AppContext>>,
    authenticated_user: AuthenticatedUser,
    move_request: web::Json<CopyRequest>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let files = postgres_service.get_multiple_files(
        move_request.file_ids.iter().map(
            |id| id.as_str()
        ).collect::<Vec<&str>>(),
        &authenticated_user.id
    ).await;

    if files.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let files = files.unwrap();

    for file in files.clone() {
        let source_key = build_key_from_path(
            &authenticated_user,
            &file.path,
        );

        let destination_key = build_key(
            &authenticated_user,
            Some(move_request.destination_path.as_str()),
            &file.id,
        );

        let res = s3_service.move_file(
            &source_key,
            &destination_key,
        ).await;

        if res.is_err() {
            return HttpResponse::InternalServerError().body("Failed to move files in storage.");
        }
    }

    let res = postgres_service.move_multiple(
        files.iter().map(|file| file.clone()).collect(),
        &move_request.destination_path,
    ).await;

    if res.is_err() {
        return HttpResponse::InternalServerError().body("Failed to move files from database.");
    }

    HttpResponse::Ok().finish()
}