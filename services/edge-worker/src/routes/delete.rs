use crate::{authenticate, AppState};
use common::types::file::delete::DeleteFilesRequest;
use std::sync::Arc;
use worker::{Method, Request, Response, RouteContext};

pub async fn handle_delete(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = &ctx.data;

    let payload: DeleteFilesRequest = req.json().await?;

    let response = state.config.make_internal_request::<_, serde_json::Value>(
        "/internal/file/delete",
        &user,
        Method::Delete,
        &payload
    ).await?;

    if response.0 == 200 {
        let kv = ctx.kv("DOWNLOAD_SESSIONS")?;

        for file_id in payload.file_ids {
            let file_lookup_key = format!("file_map:{}:{}", user.id, file_id);

            kv.delete(&file_lookup_key).await?;
        }
    }

    Ok(Response::empty()?.with_status(204))
}
