use crate::types::configuration::Configuration;
use common::types::file::upload_init::{InitUploadInternalRequest, InitUploadRequest};
use std::sync::Arc;
use worker::{Context, Env, Request, Response, Router, console_error, console_log, event};

pub mod authentication;
pub mod routes;
pub mod types;

struct AppState {
    config: Configuration,
    ctx: Context,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response, worker::Error> {
    let state = Arc::new(AppState {
        config: Configuration::gather_configuration(env.clone()),
        ctx,
    });
    let router = Router::with_data(state.clone());

    router
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
        .await
}
