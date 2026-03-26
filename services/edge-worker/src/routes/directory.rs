use crate::{authenticate, AppState};
use common::types::file::directory::{DirectoryRequest, DirectoryResponse};
use std::sync::Arc;
use worker::{Method, Request, Response, RouteContext};
use common::types::file::directory_delete::DeleteDirectoryRequest;

pub async fn handle_directory(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = &ctx.data;

    let payload: DirectoryRequest = req.json().await?;

    let response = state.config.make_internal_request::<_, DirectoryResponse>(
        "/internal/file/directory/create",
        &user.id,
        Method::Post,
        &payload
    ).await?;

    Ok(Response::from_json(&response)?)
}


pub async fn handle_directory_delete(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = &ctx.data;

    let payload: DeleteDirectoryRequest = req.json().await?;

    let response = state.config.make_internal_request::<_, ()>(
        "/internal/file/directory/delete",
        &user.id,
        Method::Delete,
        &payload
    ).await?;

    Ok(Response::from_json(&response)?)
}
