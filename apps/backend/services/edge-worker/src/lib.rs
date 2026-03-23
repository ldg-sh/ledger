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
        config: Configuration::gather_configuration(),
        ctx,
    });
    let router = Router::with_data(state.clone());

    router
        .post_async("/upload/create", |mut req, route_ctx| async move {
            let user = authenticate!(&req, &route_ctx);
            let state = route_ctx.data.clone();

            let req_body = req.json::<InitUploadRequest>().await?;
            let file_id = uuid::Uuid::new_v4();

            let res =
                match routes::upload::handle_create(req, route_ctx, &user, file_id.clone()).await {
                    Ok(res) => res,
                    Err(err) => {
                        console_log!("{:?}", err);
                        return Response::error(format!("{:?}", err), 400);
                    }
                };

            let internal_req = InitUploadInternalRequest {
                filename: req_body.filename.clone(),
                size: req_body.size,
                content_type: req_body.content_type.clone(),
                user_id: user.id.clone(),
                path: req_body.path.clone(),
                file_id: file_id.to_string(),
            };

            let background_state = state.clone();
            state.ctx.wait_until(async move {
                let result = background_state
                    .config
                    .make_internal_request::<_, serde_json::Value>(
                        "/internal/upload/init",
                        &user.id,
                        &internal_req,
                    )
                    .await;

                if let Err(e) = result {
                    console_error!("Background internal request failed: {}", e);
                }
            });

            Ok(res)
        })
        .post_async("/upload/complete", routes::upload::handle_complete)
        .post_async("/download/create", routes::download::handle_create)
        .get_async("/file/metadata", routes::metadata::handle_metadata)
        .get_async("/file/copy", routes::copy::handle_copy)
        .get_async("/file/delete", routes::delete::handle_delete)
        .get_async("/file/move", routes::r#move::handle_move)
        .get_async("/file/rename", routes::rename::handle_rename)
        .get_async("/file/directory", routes::directory::handle_directory)
        .get_async("/file/list", routes::list::handle_list)
        .get_async("/user/info", routes::user::handle_info)
        .run(req, env)
        .await
}
