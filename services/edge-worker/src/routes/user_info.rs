use crate::{AppState, try_authenticate};
use common::types::user::user_info::{UserInfoPublicResponse, UserInfoRequest, UserInfoResponse};
use serde_json::Value;
use std::sync::Arc;
use worker::*;

pub async fn handle_info(req: Request, ctx: RouteContext<Arc<AppState>>) -> Result<Response> {
    handle_info_inner(req, &ctx).await
}

pub async fn handle_info_inner(mut req: Request, ctx: &RouteContext<Arc<AppState>>) -> Result<Response> {
    let user = try_authenticate!(&req, &ctx).await;
    let payload: Result<UserInfoRequest> = req.json().await;

    let user_id = if let Ok(user) = &user {
        &user.id
    } else if let Ok(payload) = &payload {
        &payload.account_id
    } else {
        return Response::error("Missing account_id in request body and no valid authentication token provided", 400);
    };

    let state = ctx.data.clone();
    let kv = ctx.env.kv("USER_CACHE")?;

    let cache_key = format!("user:{}", user_id);

    if let Some(cached) = kv.get(&cache_key).json::<UserInfoResponse>().await? {
        return if user.is_err() {
            let public_metadata: UserInfoPublicResponse = UserInfoPublicResponse {
                id: cached.id,
                username: cached.username,
                avatar_url: cached.avatar_url,
            };

            Response::from_json(&public_metadata)
        } else {
            Response::from_json(&cached)
        }
    }

    let payload = UserInfoRequest {
        account_id: user_id.clone(),
    };

    let metadata = state
        .config
        .make_unauthenticated_internal_request::<_, Value>("/internal/user/info", Method::Post, &payload, None)
        .await?;

    if metadata.0 != 200 {
        return Ok(Response::from_json(&metadata.1)?.with_status(metadata.0));
    }
    
    let metadata: UserInfoResponse = serde_json::from_value(metadata.1)?;

    kv.put(&cache_key, &metadata)?
        .expiration_ttl(3600)
        .execute()
        .await?;

    if user.is_err() {
        let public_metadata: UserInfoPublicResponse = UserInfoPublicResponse {
            id: metadata.id,
            username: metadata.username,
            avatar_url: metadata.avatar_url,
        };

        Ok(Response::from_json(&public_metadata)?.with_status(200))
    } else {
        Ok(Response::from_json(&metadata)?.with_status(200))
    }

}
