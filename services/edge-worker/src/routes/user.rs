use crate::{authenticate, AppState};
use common::types::user_info::{UserInfoRequest, UserInfoResponse};
use serde_json::Value;
use std::sync::Arc;
use worker::*;

pub async fn handle_info(req: Request, ctx: RouteContext<Arc<AppState>>) -> Result<Response> {
    let user = authenticate!(&req, &ctx);
    let state = ctx.data;
    let kv = ctx.env.kv("USER_CACHE")?;

    let cache_key = format!("user:{}", user.id);

    if let Some(cached) = kv.get(&cache_key).json::<UserInfoResponse>().await? {
        return Response::from_json(&cached);
    }

    let user_request = UserInfoRequest {
        account_id: user.id.clone(),
    };

    let metadata = state
        .config
        .make_internal_request::<_, Value>("/internal/user/info", &user.id, Method::Post, &user_request)
        .await?;

    if metadata.0 != 200 {
        return Ok(Response::from_json(&metadata.1)?.with_status(metadata.0));
    }

    let metadata: UserInfoResponse = serde_json::from_value(metadata.1)?;

    kv.put(&cache_key, &metadata)?
        .expiration_ttl(300)
        .execute()
        .await?;

    Ok(Response::from_json(&metadata)?.with_status(200))
}
