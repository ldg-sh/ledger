use crate::AppState;
use serde_json::Value;
use std::sync::Arc;
use worker::*;

pub async fn handle_logout(req: Request, ctx: RouteContext<Arc<AppState>>) -> Result<Response> {
    let state = ctx.data;

    let (status, body, internal_headers) = state
        .config
        .make_unauthenticated_internal_request::<_, Value>(
            "/internal/user/logout",
            Method::Post,
            &serde_json::json!({}),
            Some(req.headers())
        )
        .await?;

    let mut final_res = Response::from_json(&body)?.with_status(status);

    if let Ok(all_cookies) = internal_headers.get_all("Set-Cookie") {
        for cookie in all_cookies {
            final_res.headers_mut().append("Set-Cookie", &cookie)?;
        }
    }

    Ok(final_res.with_status(200))
}