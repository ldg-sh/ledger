use worker::*;
use std::str::FromStr;
use std::time::Duration;
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use crate::authentication::authentication::get_authenticated_user;
use crate::types::configuration::Configuration;
use common::types::file::download_init::{InitDownloadRequest, InitDownloadResponse};

pub async fn handle_create(mut req: Request, ctx: RouteContext<Configuration>) -> Result<Response> {
    let authenticated_user = match get_authenticated_user(&req, &ctx).await {
        Ok(user) => user,
        Err(e) => return Ok(e.into()),
    };

    let account_id = ctx.data.r2_account_id;
    let access_key = ctx.data.r2_access_key;
    let secret_key = ctx.data.r2_secret_key;
    let bucket_name = ctx.data.r2_bucket;

    let url = format!("https://{}.r2.cloudflarestorage.com", account_id);

    let req_body = req.json::<InitDownloadRequest>().await?;

    let bucket = Bucket::new(Url::from_str(&url).unwrap(), UrlStyle::Path, bucket_name, "auto").unwrap();

    let credentials = Credentials::new(access_key.as_str(), secret_key.as_str());

    let presigned_url_duration = Duration::from_secs(60 * 60);

    let url = format!("{}/{}", authenticated_user.id, req_body.file_id);

    let action = bucket.get_object(Some(&credentials), url.as_str());
    let presigned_url = action.sign(presigned_url_duration);

    Response::from_json(&InitDownloadResponse {
        download_url: presigned_url.to_string(),
    })
}