use crate::types::configuration::Configuration;
use common::types::file::metadata::{MetadataRequest, MetadataResponse};
use worker::*;

pub async fn handle_metadata(
    mut req: Request,
    ctx: RouteContext<Configuration>,
) -> Result<Response> {
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
}
