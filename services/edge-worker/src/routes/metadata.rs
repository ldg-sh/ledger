use crate::{AppState, authenticate};
use common::types::file::metadata::{MetadataRequest, MetadataResponse};
use std::sync::Arc;
use worker::*;

pub async fn handle_metadata(
    req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> Result<Response> {
    handle_metadata_inner(req, &ctx).await
}

pub async fn handle_metadata_inner(
    mut req: Request,
    ctx: &RouteContext<Arc<AppState>>,
) -> Result<Response> {
    let user = authenticate!(&req, &ctx);

    let state = &ctx.data;
    let kv = ctx.env.kv("METADATA_CACHE")?;

    let payload: MetadataRequest = req.json().await?;
    let cache_key = format!("file:{}", payload.file_id);

    if let Some(cached) = kv.get(&cache_key).json::<MetadataResponse>().await? {
        return Response::from_json(&cached);
    }

    let metadata: (u16, MetadataResponse) = state
        .config
        .make_internal_request::<_, MetadataResponse>("/internal/file/metadata", &user, Method::Post, &payload)
        .await?;

    if metadata.0 != 200 {
        return Ok(Response::empty()?.with_status(metadata.0));
    }

    kv.put(&cache_key, &metadata.1)?
        .expiration_ttl(3600)
        .execute()
        .await?;

    Ok(Response::from_json(&metadata.1)?.with_status(metadata.0))
}
