use crate::{AppState, authenticate};
use common::types::file::file_claims::FileShare;
use common::types::file::metadata::MetadataResponse;
use common::types::file::share::{ShareRequest, ShareResponse};
use common::types::user::user_info::UserInfoResponse;
use std::sync::Arc;
use worker::{Request, Response, RouteContext};

pub async fn handle_share(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> worker::Result<Response> {
    let user = authenticate!(&req, &ctx);

    let req_for_metadata = req.clone()?;
    let req_for_info = req_for_metadata.clone()?;

    let payload: ShareRequest = req.json().await?;

    let kv = ctx.kv("DOWNLOAD_SESSIONS")?;
    let file_lookup_key = format!("file_map:{}:{}", user.id, payload.file_id);

    let mut info_response = crate::routes::user_info::handle_info_inner(req_for_info, &ctx).await?;
    let mut metadata_response = crate::routes::metadata::handle_metadata_inner(req_for_metadata, &ctx).await?;

    if info_response.status_code() != 200 {
        return Ok(info_response);
    }
    if metadata_response.status_code() != 200 {
        return Ok(metadata_response);
    }

    let user_info: UserInfoResponse = info_response.json().await?;
    let metadata: MetadataResponse = metadata_response.json().await?;

    let claims = FileShare {
        file_id: payload.file_id,
        file_name: metadata.file_name,
        owner_id: user.id,
        file_type: metadata.content_type,
        file_size: metadata.size,
        created_at: metadata.created_at,
        owner: user_info.username,
    };

    if let Some(existing_token) = kv.get(&file_lookup_key).text().await? {
        kv.put(&existing_token, serde_json::to_string(&claims)?)?
            .execute()
            .await?;
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