use serde::Serialize;
use serde::de::DeserializeOwned;
use worker::{Env, Fetch, Headers, Method, Request, RequestInit};

#[derive(Debug, Clone)]
pub struct Configuration {
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub bucket: String,
    pub jwt_secret: String,
    pub auth_server_uri: String,
}

pub struct InternalResponse<R> {
    pub status: u16,
    pub data: R,
}

impl Configuration {
    pub fn gather_configuration(env: Env) -> Configuration {
        let config = Configuration {
            access_key: env.var("ACCESS_KEY").unwrap().to_string(),
            secret_key: env.var("SECRET_KEY").unwrap().to_string(),
            endpoint: env.var("ENDPOINT").unwrap().to_string(),
            bucket: env.var("BUCKET").unwrap().to_string(),
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
    ) -> Result<(u16, serde_json::Value), worker::Error> {
        let headers = Headers::new();
        headers.set("Content-Type", "application/json")?;
        headers.set("x-user-id", user_id)?;

        let url = format!("{}{}", self.auth_server_uri, path);

        let request = Request::new_with_init(
            &url,
            &RequestInit::new()
                .with_body(Some(serde_json::to_string(payload)?.into()))
                .with_headers(headers)
                .with_method(method),
        )?;

        let mut response = Fetch::Request(request).send().await?;
        let status = response.status_code();
        let text = response.text().await?;

        let json_body: serde_json::Value = if text.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_str(&text)
                .map_err(|e| worker::Error::from(format!("Raw body was: {}. Error: {}", text, e)))?
        };

        Ok((status, json_body))
    }
}
