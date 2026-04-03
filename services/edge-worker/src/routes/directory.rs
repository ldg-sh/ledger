use crate::{authenticate, AppState};
use common::types::file::directory::DirectoryRequest;
use common::types::file::directory_delete::DeleteDirectoryRequest;
use serde_json::Value;
use std::sync::Arc;
use worker::{Method, Request, Response, RouteContext};

pub async fn handle_directory(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = &ctx.data;

    let payload: DirectoryRequest = req.json().await?;

    let response = state.config.make_internal_request::<_, Value>(
        "/internal/file/directory/create",
        &user.id,
        Method::Post,
        &payload
    ).await?;
    
    if response.0 != 200 {
        return Ok(Response::from_json(&response.1)?.with_status(response.0));
    }

    Ok(Response::from_json(&response.1)?.with_status(200))
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

    Ok(Response::from_json(&response.1)?.with_status(response.0))
}
