use crate::{authenticate, AppState};
use common::types::file::delete::DeleteFilesRequest;
use std::sync::Arc;
use worker::{Request, Response, RouteContext};

pub async fn handle_delete(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = &ctx.data;

    let payload: DeleteFilesRequest = req.json().await?;

    let _response = state.config.make_internal_request::<_, serde_json::Value>(
        "/internal/file/copy",
        &user.id,
        &payload
    ).await?;

    Ok(Response::ok("Successfully deleted files")?)
}
