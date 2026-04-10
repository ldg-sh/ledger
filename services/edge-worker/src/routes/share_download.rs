use crate::AppState;
use common::types::file::share::{ShareDownloadRequest, ShareDownloadResponse};
use common::types::file_claims::FileClaims;
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use worker::*;

pub async fn handle_share_download(
    mut req: Request,
    ctx: RouteContext<Arc<AppState>>,
) -> Result<Response, Error> {
    let state = ctx.data.clone();

    let payload: ShareDownloadRequest = req.json().await?;

    let kv = ctx.kv("DOWNLOAD_SESSIONS")?;

    let claims = match kv.get(&payload.token).json::<FileClaims>().await? {
        Some(c) => c,
        None => return Response::error("Link expired or invalid", 410),
    };

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
    let s3_path = format!("{}/{}", claims.owner_id, claims.file_id);

    let mut action = bucket.get_object(Some(&credentials), &s3_path);
    action.query_mut().insert(
        "response-content-disposition",
        format!("attachment; filename=\"{}\"", claims.file_name),
    );

    let presigned_url = action.sign(Duration::from_secs(300));

    Response::from_json(&ShareDownloadResponse {
        presigned_url: presigned_url.to_string(),
        file_type: claims.file_type
    })
}
