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
    let origin = req.headers().get("Origin")?.unwrap_or_default();
    let allowed_origin = env.var("ALLOWED_ORIGIN")?.to_string();

    let is_allowed = origin == allowed_origin || origin == "http://localhost:3000";

    if req.method() == worker::Method::Options {
        let headers = worker::Headers::new();
        if is_allowed {
            headers.set("Access-Control-Allow-Origin", &origin)?;
            headers.set("Access-Control-Allow-Methods", "GET, POST, DELETE, OPTIONS, PATCH")?;
            headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
            headers.set("Access-Control-Allow-Credentials", "true")?;
            headers.set("Vary", "Origin")?;
        }
        return Ok(Response::empty()?.with_headers(headers));
    }

    let state = Arc::new(AppState {
        config: Configuration::gather_configuration(env.clone()),
    });

    let mut response = Router::with_data(state.clone())
        .post_async("/upload/create", routes::upload::handle_create)
        .post_async("/upload/complete", routes::upload::handle_complete)
        .post_async("/download/create", routes::download::handle_create)
        .post_async("/file/metadata", routes::metadata::handle_metadata)
        .post_async("/file/copy", routes::copy::handle_copy)
        .delete_async("/file/delete", routes::delete::handle_delete)
        .post_async("/file/move", routes::r#move::handle_move)
        .post_async("/file/rename", routes::rename::handle_rename)
        .post_async("/file/zip", routes::zip::handle_zip)
        .post_async("/directory/create", routes::directory::handle_directory)
        .delete_async("/directory/delete", routes::directory::handle_directory_delete)
        .post_async("/file/list", routes::list::handle_list)
        .get_async("/user/info", routes::user::handle_info)
        .run(req, env)
        .await?;

    if is_allowed {
        let headers = response.headers_mut();
        headers.set("Access-Control-Allow-Origin", &origin)?;
        headers.set("Access-Control-Allow-Credentials", "true")?;
        headers.set("Vary", "Origin")?;
    }

    Ok(response)
}