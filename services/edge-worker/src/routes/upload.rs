use crate::authentication::authentication::AuthenticatedUser;
use crate::{AppState, authenticate, routes};
use common::types::file::upload_complete::CompleteUploadRequest;
use common::types::file::upload_init::{
    InitUploadInternalRequest, InitUploadInternalResponse, InitUploadRequest, InitUploadResponse,
};
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;
use worker::*;

pub async fn handle_create(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> Result<Response> {
    let user = authenticate!(&req, &ctx);

    let state = ctx.data;

    let account_id = state.config.r2_account_id.clone();
    let access_key = state.config.r2_access_key.clone();
    let secret_key = state.config.r2_secret_key.clone();
    let bucket_name = state.config.r2_bucket.clone();
    let url = format!("https://{}.r2.cloudflarestorage.com", account_id);

    let bucket = Bucket::new(Url::from_str(&url)?, UrlStyle::Path, bucket_name, "auto").unwrap();

    let req_body = &req.json::<InitUploadRequest>().await?;
    let file_id = Uuid::new_v4();

    let internal_req = InitUploadInternalRequest {
        filename: req_body.filename.clone(),
        size: req_body.size,
        content_type: req_body.content_type.clone(),
        user_id: user.id.clone(),
        path: req_body.path.clone(),
        file_id: file_id.to_string(),
    };

    let background_state = state.clone();
    let result = background_state
        .config
        .make_internal_request::<_, InitUploadInternalResponse>(
            "/internal/upload/init",
            &user.id,
            &internal_req,
        )
        .await?;

    let credentials = Credentials::new(access_key.as_str(), secret_key.as_str());

    let presigned_url_duration = Duration::from_secs(60 * 60);

    let url = format!("{}/{}", user.id, file_id);

    let action = bucket.put_object(Some(&credentials), url.as_str());
    let presigned_url = action.sign(presigned_url_duration);

    Response::from_json(&InitUploadResponse {
        file_id: file_id.to_string(),
        upload_url: presigned_url.to_string(),
        upload_id: result.upload_id,
    })
}

pub async fn handle_complete(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> Result<Response> {
    let user = authenticate!(&req, &ctx);

    let state = ctx.data;
    let req_body = req.json::<CompleteUploadRequest>().await?;

    match state
        .config
        .make_internal_request::<_, serde_json::Value>(
            "/internal/upload/complete",
            &user.id,
            &req_body,
        )
        .await
    {
        Ok(_) => Response::ok(""),
        Err(e) => Response::error(e.to_string(), 500),
    }
}
