use std::sync::Arc;
use actix_web::{delete, patch, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::util::file::{build_key, build_key_from_path};

#[derive(Serialize, Deserialize)]
pub struct CopyRequest {
    #[serde(rename = "destinationPath")]
    pub destination_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct RenameRequest {
    #[serde(rename = "newName")]
    pub new_name: String,
}

#[post("/copy")]
pub async fn copy(
    context: web::Data<Arc<AppContext>>,
    file_id: web::Path<String>,
    authenticated_user: AuthenticatedUser,
    copy_request: web::Json<CopyRequest>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let file = postgres_service.get_file(
        &file_id.clone(),
        &authenticated_user.id
    ).await;

    if file.is_err() {
        return HttpResponse::InternalServerError()
            .body(format!("{:?}", file.unwrap_err()));
    }

    if file.as_ref().unwrap().is_none() {
        return HttpResponse::NotFound().finish();
    }

    let file = file.unwrap().unwrap();

    let original_path = file.path.clone();
    let new_file_id = uuid::Uuid::new_v4().to_string();

    let source_key = build_key_from_path(
        &authenticated_user,
        &original_path,
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

    let copy = postgres_service.copy_file(
        file,
        &new_file_id,
        &copy_request.destination_path,
    ).await;

    if copy.is_err() {
        return HttpResponse::InternalServerError().body(
            "File copied in storage but failed to update database."
        )
    }

    HttpResponse::Ok().json(serde_json::json!({
        "file_id": new_file_id,
    }))
}

#[post("/move")]
pub async fn r#move(
    context: web::Data<Arc<AppContext>>,
    file_id: web::Path<String>,
    authenticated_user: AuthenticatedUser,
    move_request: web::Json<CopyRequest>,
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let file = postgres_service.get_file(
        &file_id.clone(),
        &authenticated_user.id
    ).await;

    if file.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if file.as_ref().unwrap().is_none() {
        return HttpResponse::NotFound().finish();
    }

    let file = file.unwrap().unwrap();
    let original_path = file.path.clone();

    let source_key = build_key_from_path(
        &authenticated_user,
        &original_path,
    );

    let destination_key = build_key_from_path(
        &authenticated_user,
        &move_request.destination_path,
    );

    let s3_move = s3_service.move_file(
        &source_key,
        &destination_key,
    ).await;

    if s3_move.is_err() {
        return HttpResponse::InternalServerError().body(
            "Failed to move file in storage."
        )
    }

    let move_result = postgres_service.move_file(
        file,
        &move_request.destination_path,
    ).await;

    if move_result.is_err() {
        let _ = s3_service.move_file(
            &destination_key,
            &source_key,
        ).await;

        return HttpResponse::InternalServerError().body(
            "File moved in storage but failed to update database."
        )
    }

    HttpResponse::Ok().finish()
}

#[delete("")]
pub async fn delete_file(
    context: web::Data<Arc<AppContext>>,
    file_id: web::Path<String>,
    authenticated_user: AuthenticatedUser
) -> HttpResponse {
    let s3_service = Arc::clone(&context.clone().into_inner().s3_service);
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let file = postgres_service.get_file(
        &file_id.clone(),
        &authenticated_user.id
    ).await;

    if file.is_err() {
        return HttpResponse::InternalServerError().body(
            "Failed to retrieve file from database."
        )
    }

    if file.as_ref().unwrap().is_none() {
        return HttpResponse::NotFound().finish();
    }

    let file = file.unwrap().unwrap();
    let path = build_key_from_path(
        &authenticated_user,
        &file.path,
    );

    let s3_delete = s3_service.delete(&path).await;

    if s3_delete.is_err() {
        return HttpResponse::InternalServerError().body(
            "Failed to delete file from storage."
        )
    }

    let db_delete = postgres_service.delete_file(
        file
    ).await;

    if db_delete.is_err() {
        return HttpResponse::InternalServerError().body(
            "File deleted from storage but failed to delete from database."
        )
    }

    HttpResponse::Ok().finish()
}

#[patch("")]
pub async fn rename_file(
    context: web::Data<Arc<AppContext>>,
    file_id: web::Path<String>,
    authenticated_user: AuthenticatedUser,
    rename_request: web::Json<RenameRequest>,
) -> HttpResponse {
    let postgres_service = Arc::clone(&context.into_inner().postgres_service);

    let file = postgres_service.get_file(
        &file_id.clone(),
        &authenticated_user.id
    ).await;

    if file.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if file.as_ref().unwrap().is_none() {
        return HttpResponse::NotFound().finish();
    }

    let file = file.unwrap().unwrap();

    let rename_result = postgres_service.rename_file(
        file,
        &rename_request.new_name,
    ).await;

    if rename_result.is_err() {
        return HttpResponse::InternalServerError().body(
            "Failed to rename file in database."
        )
    }

    HttpResponse::Ok().finish()
}