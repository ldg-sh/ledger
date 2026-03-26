use crate::{AppState, authenticate};
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

pub async fn handle_create(mut req: Request, ctx: RouteContext<Arc<AppState>>) -> Result<Response> {
    let user = authenticate!(&req, &ctx);

    let state = ctx.data;

    let access_key = state.config.access_key.clone();
    let secret_key = state.config.secret_key.clone();
    let bucket_name = state.config.bucket.clone();
    let url = state.config.endpoint.clone();

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
            Method::Post,
            &internal_req,
        )
        .await?;

    let credentials = Credentials::new(access_key.as_str(), secret_key.as_str());

    let presigned_url_duration = Duration::from_secs(60 * 60);
    let url = format!("{}/{}", user.id, file_id);

    let mut urls = vec![];
    for i in 1..=req_body.part_count {
        let action = bucket.upload_part(
            Some(&credentials),
            url.as_str(),
            i as u16,
            result.upload_id.as_str(),
        );
        let presigned_url = action.sign(presigned_url_duration);
        urls.push(presigned_url.to_string());
    }

    Response::from_json(&InitUploadResponse {
        file_id: file_id.to_string(),
        upload_urls: urls,
        upload_id: result.upload_id,
    })
}

pub async fn handle_complete(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> Result<Response> {
    let user = authenticate!(&req, &ctx);

    let state = ctx.data;
    let body = req.text().await?;

    let req_body = match serde_json::from_str::<CompleteUploadRequest>(&body) {
        Ok(data) => data,
        Err(_) => {
            return Response::error("Invalid request body", 400);
        }
    };

    match state
        .config
        .make_internal_request::<_, ()>(
            "/internal/upload/complete",
            &user.id,
            Method::Post,
            &req_body,
        )
        .await
    {
        Ok(_) => Response::ok(""),
        Err(error) => {
            Response::error(error.to_string(), 500)
        }
    }
}
