use crate::{authenticate, AppState};
use common::types::file::r#move::MoveFilesRequest;
use std::sync::Arc;
use worker::{Method, Request, Response, RouteContext};

pub async fn handle_move(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = &ctx.data;

    let payload: MoveFilesRequest = req.json().await?;

    let response = state.config.make_internal_request::<_, serde_json::Value>(
        "/internal/file/move",
        &user.id,
        Method::Post,
        &payload
    ).await?;

    Ok(Response::from_json(&response)?)
}
