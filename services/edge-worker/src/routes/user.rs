use crate::{AppState, authenticate};
use common::types::user_info::{UserInfoRequest, UserInfoResponse};
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

    let metadata: UserInfoResponse = state
        .config
        .make_internal_request("/internal/user/info", &user.id, Method::Post, &user_request)
        .await?;

    kv.put(&cache_key, &metadata)?
        .expiration_ttl(300)
        .execute()
        .await?;

    Response::from_json(&metadata)
}
