use crate::{authenticate, AppState};
use common::types::file::metadata::{MetadataRequest, MetadataResponse};
use serde_json::Value;
use std::sync::Arc;
use worker::*;

pub async fn handle_metadata(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> Result<Response> {
    let user = authenticate!(&req, &ctx);

    let state = &ctx.data;
    let kv = ctx.env.kv("METADATA_CACHE")?;

    let payload: MetadataRequest = req.json().await?;
    let cache_key = format!("file:{}", payload.file_id);

    if let Some(cached) = kv.get(&cache_key).json::<MetadataResponse>().await? {
        return Response::from_json(&cached);
    }

    let metadata = state
        .config
        .make_internal_request::<_, Value>("/internal/file/metadata", &user.id, Method::Post, &payload)
        .await?;

    kv.put(&cache_key, &metadata.1)?
        .expiration_ttl(3600)
        .execute()
        .await?;

    Ok(Response::from_json(&metadata.1)?.with_status(metadata.0))
}
