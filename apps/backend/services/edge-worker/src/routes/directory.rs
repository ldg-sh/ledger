use crate::{authenticate, AppState};
use common::types::file::directory::{DirectoryRequest, DirectoryResponse};
use std::sync::Arc;
use worker::{Request, Response, RouteContext};

pub async fn handle_directory(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = &ctx.data;

    let payload: DirectoryRequest = req.json().await?;

    let response = state.config.make_internal_request::<_, DirectoryResponse>(
        "/internal/file/directory",
        &user.id,
        &payload
    ).await?;

    Ok(Response::from_json(&response)?)
}
