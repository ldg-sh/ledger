use std::sync::Arc;
use actix_web::{get, web, HttpResponse, Responder};
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;


#[derive(serde::Serialize)]
struct AllFilesSummary {
    files: Vec<FileSummary>,
}

#[derive(serde::Serialize)]
struct FileSummary {
    file_id: String,
    file_name: String,
    file_size: i64,
    file_type: String,
    created_at: chrono::DateTime<chrono::Utc>,
    path: String,
}

#[get("/{path:.*}")]
pub async fn list_files(
    context: web::Data<Arc<AppContext>>,
    path: Option<web::Path<String>>,
    authenticated_user: AuthenticatedUser,
) -> impl Responder {
    let postgres = Arc::clone(&context.clone().into_inner().postgres_service);

    let path_str = if path.is_none() {
        "".to_string()
    } else {
        let path_value = path.as_ref().unwrap();
        let mut p = path_value.to_string();

        if p.ends_with('/') {
            p.pop();
        }

        if p.starts_with('/') {
            p = p.replacen('/', "", 1);
        }

        p
    };

    let files = postgres.list_related_files(
        &path_str,
        &authenticated_user.id
    ).await;


    if let Ok(files) = files {
        let files: Vec<_> = files
            .into_iter()
            .map(|v| FileSummary {
                file_id: v.id,
                file_name: v.file_name,
                file_size: v.file_size,
                file_type: v.file_type,
                created_at: v.created_at,
                path: v.path
            })
            .collect();

        let cleaned = AllFilesSummary {
            files,
        };

        return HttpResponse::Ok().json(cleaned);
    }

    HttpResponse::Ok().finish()
}
