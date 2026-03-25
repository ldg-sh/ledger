use crate::types::configuration::Configuration;
use std::sync::Arc;
use worker::{event, Context, Env, Request, Response, Router};

pub mod authentication;
pub mod routes;
pub mod types;

struct AppState {
    config: Configuration,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response, worker::Error> {
    let allowed_origins = [
        env.var("ALLOWED_ORIGIN")?.to_string(),
        "http://localhost:3000".to_string(),
    ];
    let allowed_origins_ref: Vec<&str> = allowed_origins.iter().map(|s| s.as_str()).collect();

    let is_preflight = req.headers().get("Access-Control-Request-Method").ok().flatten().is_some();
    let origin = req.headers().get("Origin").ok().flatten();

    if is_preflight {
        let response = Response::empty()?;
        let headers = response.headers().clone();
        if let Some(o) = &origin {
            if allowed_origins_ref.contains(&o.as_str()) {
                headers.set("Access-Control-Allow-Origin", o)?;
                headers.set("Access-Control-Allow-Credentials", "true")?;
                headers.set("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")?;
                headers.set("Access-Control-Allow-Headers", "Content-Type")?;
            }
        }
        return Ok(response.with_headers(headers));
    }

    let state = Arc::new(AppState {
        config: Configuration::gather_configuration(env.clone()),
    });

    let response = Router::with_data(state.clone())
        .post_async("/upload/create", routes::upload::handle_create)
        .post_async("/upload/complete", routes::upload::handle_complete)
        .post_async("/download/create", routes::download::handle_create)
        .post_async("/file/metadata", routes::metadata::handle_metadata)
        .post_async("/file/copy", routes::copy::handle_copy)
        .delete_async("/file/delete", routes::delete::handle_delete)
        .post_async("/file/move", routes::r#move::handle_move)
        .post_async("/file/rename", routes::rename::handle_rename)
        .post_async("/directory/create", routes::directory::handle_directory)
        .delete_async("/directory/delete", routes::directory::handle_directory)
        .post_async("/file/list", routes::list::handle_list)
        .get_async("/user/info", routes::user::handle_info)
        .run(req, env)
        .await?;

    let headers = response.headers().clone();
    if let Some(o) = &origin {
        if allowed_origins_ref.contains(&o.as_str()) {
            headers.set("Access-Control-Allow-Origin", o)?;
            headers.set("Access-Control-Allow-Credentials", "true")?;
            headers.set("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS")?;
            headers.set("Access-Control-Allow-Headers", "Content-Type")?;
        }
    }
    Ok(response.with_headers(headers))
}