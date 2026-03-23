use crate::authentication::authentication::get_authenticated_user;
use crate::types::configuration::Configuration;
use common::types::user_info::{UserInfoRequest, UserInfoResponse};
use worker::*;

pub async fn handle_info(req: Request, ctx: RouteContext<Configuration>) -> Result<Response> {
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
}
