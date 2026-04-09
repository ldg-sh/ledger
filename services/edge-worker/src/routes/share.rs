use crate::{AppState, authenticate};
use common::types::file::share::{ShareRequest, ShareResponse};
use common::types::file_claims::FileClaims;
use std::sync::Arc;
use uuid::Uuid;
use worker::{Request, Response, RouteContext};

pub async fn handle_share(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);

    let payload: ShareRequest = req.json().await?;

    let download_id = Uuid::new_v4().to_string();

    let claims = FileClaims {
        file_id: payload.file_id,
        file_name: payload.file_name,
        owner_id: user.id,
    };

    let kv = ctx.kv("DOWNLOAD_SESSIONS")?;

    kv.put(&download_id, serde_json::to_string(&claims)?)?
        .execute()
        .await?;

    Ok(Response::from_json(&ShareResponse { token: download_id })?)
}
