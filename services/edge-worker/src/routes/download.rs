use crate::{authenticate, AppState};
use common::types::file::download_init::{InitDownloadRequest, InitDownloadResponse};
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use worker::*;

pub async fn handle_create(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> Result<Response> {
    let authenticated_user = authenticate!(&req, &ctx);

    let state = ctx.data;
    
    let account_id = state.config.r2_account_id.clone();
    let access_key = state.config.r2_access_key.clone();
    let secret_key = state.config.r2_secret_key.clone();
    let bucket_name = state.config.r2_bucket.clone();

    let url = format!("https://{}.r2.cloudflarestorage.com", account_id);

    let req_body = req.json::<InitDownloadRequest>().await?;

    let bucket = Bucket::new(Url::from_str(&url)?, UrlStyle::Path, bucket_name, "auto").unwrap();

    let credentials = Credentials::new(access_key.as_str(), secret_key.as_str());

    let presigned_url_duration = Duration::from_secs(60 * 60);

    let url = format!("{}/{}", authenticated_user.id, req_body.file_id);

    let action = bucket.get_object(Some(&credentials), url.as_str());
    let presigned_url = action.sign(presigned_url_duration);

    Response::from_json(&InitDownloadResponse {
        download_url: presigned_url.to_string(),
    })
}