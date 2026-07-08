use crate::{AppState, authenticate};
use common::types::file::file_claims::FileShare;
use common::types::file::share::{ShareRequest, ShareResponse};
use std::sync::Arc;
use worker::{Request, Response, RouteContext};

pub async fn handle_share(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);

    let payload: ShareRequest = req.json().await?;

    let kv = ctx.kv("DOWNLOAD_SESSIONS")?;
    let file_lookup_key = format!("file_map:{}:{}", user.id, payload.file_id);

    let claims = FileShare {
        file_id: payload.file_id,
    };

    if let Some(existing_token) = kv.get(&file_lookup_key).text().await? {
        return Ok(Response::from_json(&ShareResponse {
            token: existing_token,
        })?);
    }

    let download_id = nanoid::nanoid!(8);

    kv.put(&download_id, serde_json::to_string(&claims)?)?
        .execute()
        .await?;
    kv.put(&file_lookup_key, &download_id)?.execute().await?;

    Ok(Response::from_json(&ShareResponse { token: download_id })?)
}