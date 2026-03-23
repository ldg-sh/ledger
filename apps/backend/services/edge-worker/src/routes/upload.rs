use crate::authentication::authentication::get_authenticated_user;
use crate::types::configuration::Configuration;
use common::types::file::upload_complete::CompleteUploadRequest;
use common::types::file::upload_init::{InitUploadRequest, InitUploadResponse};
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use std::str::FromStr;
use std::time::Duration;
use worker::*;

pub async fn handle_create(mut req: Request, ctx: RouteContext<Configuration>) -> Result<Response> {
    match get_authenticated_user(&req, &ctx).await {
        Ok(user) => user,
        Err(e) => return Ok(e.into()),
    };

    let config = &ctx.data;

    let account_id = config.r2_account_id.clone();
    let access_key = config.r2_access_key.clone();
    let secret_key = config.r2_secret_key.clone();
    let bucket_name = config.r2_bucket.clone();
    let url = format!("https://{}.r2.cloudflarestorage.com", account_id);

    let req_body = req.json::<InitUploadRequest>().await?;

    let bucket = Bucket::new(
        Url::from_str(&url).unwrap(),
        UrlStyle::Path,
        bucket_name,
        "auto",
    )
    .unwrap();

    let credentials = Credentials::new(access_key.as_str(), secret_key.as_str());

    let presigned_url_duration = Duration::from_secs(60 * 60);

    let uuid = uuid::Uuid::new_v4().to_string();
    let url = format!("{}/{}", req_body.user_id, uuid.as_str());

    let action = bucket.put_object(Some(&credentials), url.as_str());
    let presigned_url = action.sign(presigned_url_duration);

    Response::from_json(&InitUploadResponse {
        file_id: uuid,
        upload_url: presigned_url.to_string(),
    })
}

pub async fn handle_complete(
    mut req: Request,
    ctx: RouteContext<Configuration>,
) -> Result<Response> {
    match get_authenticated_user(&req, &ctx).await {
        Ok(user) => user,
        Err(e) => return Ok(e.into()),
    };

    let auth_server_uri = ctx.data.auth_server_uri;

    let req_body = req.json::<CompleteUploadRequest>().await?;

    let headers = Headers::new();
    headers
        .append("Content-Type", "application/json")
        .expect("Failed to set header");

    let request = Request::new_with_init(
        format!("{}/internal/upload/complete", auth_server_uri).as_str(),
        RequestInit::new()
            .with_body(Some(serde_json::to_string(&req_body)?.into()))
            .with_headers(headers)
            .with_method(Method::Post),
    )?;

    let response = Fetch::Request(request).send().await?;

    if response.status_code() == 200 {
        Response::ok("")
    } else {
        Response::error("Origin Error", response.status_code())
    }
}
