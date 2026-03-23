use crate::authentication::authentication::get_authenticated_user;
use crate::types::configuration::Configuration;
use common::types::file::download_init::{InitDownloadRequest, InitDownloadResponse};
use common::types::file::metadata::{MetadataRequest, MetadataResponse};
use common::types::file::upload_complete::CompleteUploadRequest;
use common::types::file::upload_init::{InitUploadRequest, InitUploadResponse};
use common::types::user_info::{UserInfoRequest, UserInfoResponse};
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use std::str::FromStr;
use std::time::Duration;
use worker::{event, Context, Env, Fetch, Headers, Method, Request, RequestInit, Response, Router, Url};

pub mod authentication;
pub mod types;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response, worker::Error> {
    let config = Configuration::gather_configuration();
    let router = Router::with_data(config);

    router
        .get_async("/metadata", |mut req, ctx| async move {
            let config = &ctx.data;

            let auth_server_uri = config.auth_server_uri.clone();
            let kv = ctx.env.kv("METADATA_CACHE")?;

            let payload: MetadataRequest = match req.json().await {
                Ok(payload) => payload,
                Err(e) => {
                    return Response::error(format!("Invalid request body: {}", e), 400);
                }
            };

            let cache_key = format!("file:{}", payload.file_id);

            if let Some(cached) = kv.get(&cache_key).json::<MetadataResponse>().await? {
                return Response::from_json(&cached);
            }

            let headers = Headers::new();
            headers
                .append("Content-Type", "application/json")
                .expect("Failed to set header");

            let request = Request::new_with_init(
                format!("{}/internal/metadata", auth_server_uri).as_str(),
                RequestInit::new()
                    .with_body(Some(serde_json::to_string(&payload)?.into()))
                    .with_headers(headers)
                    .with_method(Method::Post),
            )?;

            let mut response = Fetch::Request(request).send().await?;

            if response.status_code() == 200 {
                let metadata: MetadataResponse = response.json().await?;

                kv.put(&cache_key, &metadata)?
                    .expiration_ttl(3600)
                    .execute()
                    .await?;

                Response::from_json(&metadata)
            } else {
                Response::error("Origin Error", response.status_code())
            }
        })
        .post_async("/upload/create", |mut req, ctx| async move {
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

            let bucket = Bucket::new(Url::from_str(&url).unwrap(), UrlStyle::Path, bucket_name, "auto").unwrap();

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
        })
        .post_async("/upload/complete", |mut req, ctx| async move {
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
        })
        .post_async("/download/create", |mut req, ctx| async move {
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
        })
        .get_async("/user/info", |req, ctx| async move {
            let authenticated_user = match get_authenticated_user(&req, &ctx).await {
                Ok(user) => user,
                Err(e) => return Ok(e.into()),
            };

            let auth_server_uri = ctx.data.auth_server_uri;
            let kv = ctx.env.kv("USER_CACHE")?;

            let cache_key = format!("user:{}", authenticated_user.id);

            if let Some(cached) = kv.get(&cache_key).json::<UserInfoResponse>().await? {
                return Response::from_json(&cached);
            }

            let user_request = UserInfoRequest {
                account_id: authenticated_user.id.clone(),
            };

            let headers = Headers::new();
            headers
                .append("Content-Type", "application/json")
                .expect("Failed to set header");

            let request = Request::new_with_init(
                format!("{}/internal/user/info", auth_server_uri).as_str(),
                RequestInit::new()
                    .with_body(Some(serde_json::to_string(&user_request)?.into()))
                    .with_headers(headers)
                    .with_method(Method::Post),
            )?;

            let mut response = Fetch::Request(request).send().await?;

            if response.status_code() == 200 {
                let body_text = response.text().await?;

                let metadata: UserInfoResponse = match serde_json::from_str(&body_text) {
                    Ok(m) => m,
                    Err(_) => return Response::error(format!("Failed to parse: {}", body_text), 500),
                };

                kv.put(&cache_key, &metadata)?
                    .expiration_ttl(300)
                    .execute()
                    .await?;

                Response::from_json(&metadata)
            } else {
                let error_body = response.text().await?;
                Response::error(error_body, response.status_code())
            }
        })
        .run(req, env)
        .await
}
