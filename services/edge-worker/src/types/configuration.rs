use serde::de::DeserializeOwned;
use serde::Serialize;
use worker::{Env, Fetch, Headers, Method, Request, RequestInit};

#[derive(Debug, Clone)]
pub struct Configuration {
    pub r2_account_id: String,
    pub r2_access_key: String,
    pub r2_secret_key: String,
    pub r2_bucket: String,
    pub jwt_secret: String,
    pub auth_server_uri: String,
}

impl Configuration {
    pub fn gather_configuration(env: Env) -> Configuration {
        let config = Configuration {
            r2_account_id: env.var("R2_ACCOUNT_ID").unwrap().to_string(),
            r2_access_key: env.var("R2_ACCESS_KEY").unwrap().to_string(),
            r2_secret_key: env.var("R2_SECRET_KEY").unwrap().to_string(),
            r2_bucket: env.var("R2_BUCKET").unwrap().to_string(),
            jwt_secret: env.var("JWT_SECRET").unwrap().to_string(),
            auth_server_uri: env.var("AUTH_SERVER_URI").unwrap().to_string(),
        };

        config
    }

    pub async fn make_internal_request<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        user_id: &str,
        method: Method,
        payload: &T,
    ) -> Result<R, worker::Error> {
        let headers = Headers::new();
        headers.set("Content-Type", "application/json")?;
        headers.set("x-user-id", user_id)?;

        let url = format!("{}{}", self.auth_server_uri, path);

        let request = Request::new_with_init(
            &url,
            RequestInit::new()
                .with_body(Some(serde_json::to_string(payload)?.into()))
                .with_headers(headers)
                .with_method(method),
        )?;

        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() >= 200 && response.status_code() < 300 {
            let text = response.text().await?;

            if text.is_empty() {
                return serde_json::from_str::<R>("null")
                    .map_err(|e| worker::Error::from(e.to_string()));
            }

            serde_json::from_str::<R>(&text).map_err(|e| worker::Error::from(e.to_string()))
        } else {
            Err(worker::Error::from(format!(
                "Internal Request Failed: {} - {}",
                response.status_code(),
                response.text().await.unwrap_or_default()
            )))
        }
    }
}
