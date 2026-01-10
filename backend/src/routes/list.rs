use std::sync::Arc;
use actix_web::{get, web, HttpResponse, Responder};
use crate::context::AppContext;
use crate::middleware::authentication::AuthenticatedUser;
use crate::util::file::build_key_from_path;


#[derive(serde::Serialize)]
struct AllFilesSummary {
    files: Vec<FileSummary>,
    folders: Vec<FolderSummary>,
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

#[derive(serde::Serialize)]
struct FolderSummary {
    name: String,
    file_count: i64,
    size: i64,
}

#[get("/{path:.*}")]
pub async fn list_files(
    context: web::Data<Arc<AppContext>>,
    path: Option<web::Path<String>>,
    authenticated_user: AuthenticatedUser,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let postgres = Arc::clone(&context.clone().into_inner().postgres_service);
    let s3_service = Arc::clone(&context.into_inner().s3_service);
    let deep = query.get("deep").is_some();

    let full_path = if path.is_none() {
        build_key_from_path(&authenticated_user, "")
    } else {
        let path_value = path.as_ref().unwrap();
        build_key_from_path(&authenticated_user, &path_value)
    };

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

    let files = if deep {
        postgres.list_related_files(
            path_str.clone().as_str(),
            &authenticated_user.id
        ).await
    } else {
        postgres.list_files(
            path_str.clone().as_str(),
            &authenticated_user.id
        ).await
    };

    let folders = s3_service.list_directories(
        &full_path
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

        let mut folder_summaries = vec![];

        if let Ok(folders) = folders {
            for folder in folders {
                let folder = if !path_str.is_empty() {
                    format!("{}/{}", path_str, folder)
                } else {
                    folder
                };

                let files_in_folder = if deep {
                    postgres.list_related_files(
                        &folder,
                        &authenticated_user.id
                    ).await
                } else {
                    postgres.list_files(
                        &folder,
                        &authenticated_user.id
                    ).await
                };

                if let Ok(files_in_folder) = files_in_folder {
                    let file_count = files_in_folder.len() as i64;
                    let size: i64 = files_in_folder.iter().map(|f| f.file_size).sum();

                    folder_summaries.push(FolderSummary {
                        name: folder.replace(&format!("{}/", path_str), ""),
                        file_count,
                        size,
                    });
                }
            }
        }


        let cleaned = AllFilesSummary {
            files,
            folders: folder_summaries,
        };

        return HttpResponse::Ok().json(cleaned);
    }

    HttpResponse::Ok().finish()
}
