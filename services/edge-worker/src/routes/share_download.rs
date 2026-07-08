use crate::AppState;
use common::types::file::file_claims::FileShare;
use common::types::file::metadata::{MetadataRequest, MetadataResponse};
use common::types::file::share::{ShareDownloadRequest, ShareDownloadResponse};
use common::types::user::user_info::{UserInfoPublicResponse, UserInfoRequest};
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::JsValue;
use worker::*;

pub async fn handle_share_download(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> Result<Response, Error> {
    let state = ctx.data.clone();

    let payload: ShareDownloadRequest = req.json().await?;

    let kv = ctx.kv("DOWNLOAD_SESSIONS")?;

    let claims = match kv.get(&payload.token).json::<FileShare>().await? {
        Some(c) => c,
        None => return Response::error("Link expired or invalid", 410),
    };

    let metadata_request = MetadataRequest {
        file_id: claims.file_id.clone(),
    };

    let body_str = serde_json::to_string(&metadata_request)?;

    let mut metadata_init = RequestInit::new();
    metadata_init.with_method(Method::Post)
        .with_body(Some(JsValue::from_str(&body_str)));

    let metadata_req = Request::new_with_init("http://internal/metadata", &metadata_init)?;
    let mut metadata_response = crate::routes::metadata::handle_metadata_inner(metadata_req, &ctx).await?;

    let res = metadata_response.json::<MetadataResponse>().await?;

    let user_request = UserInfoRequest {
        account_id: res.owner_id.clone(),
    };

    let user_body_str = serde_json::to_string(&user_request)?;

    let mut user_init = RequestInit::new();
    user_init.with_method(Method::Post)
        .with_body(Some(JsValue::from_str(&user_body_str)));

    let user_req = Request::new_with_init("http://internal/user", &user_init)?;

    let mut user_req = crate::routes::user_info::handle_info_inner(user_req, &ctx).await?;
    let user_res = user_req.json::<UserInfoPublicResponse>().await?;

    let access_key = state.config.access_key.clone();
    let secret_key = state.config.secret_key.clone();
    let bucket_name = state.config.bucket.clone();
    let endpoint = state.config.endpoint.clone();

    let bucket = Bucket::new(
        Url::from_str(&endpoint)?,
        UrlStyle::Path,
        bucket_name,
        "auto",
    )
    .map_err(|e| Error::from(e.to_string()))?;

    let credentials = Credentials::new(access_key.as_str(), secret_key.as_str());
    let s3_path = format!("{}/{}", res.owner_id, claims.file_id);

    let mut action = bucket.get_object(Some(&credentials), &s3_path);
    action.query_mut().insert(
        "response-content-disposition",
        format!("attachment; filename=\"{}\"", res.file_name),
    );

    let presigned_url = action.sign(Duration::from_secs(300));

    Response::from_json(&ShareDownloadResponse {
        presigned_url: presigned_url.to_string(),
        file_type: res.content_type,
        file_name: res.file_name,
        file_size: res.size,
        created_at: res.created_at.to_string(),
        owner: user_res.username,
    })
}
