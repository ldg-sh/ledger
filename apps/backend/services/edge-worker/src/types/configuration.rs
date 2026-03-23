use std::env;

pub struct Configuration {
    pub r2_account_id: String,
    pub r2_access_key: String,
    pub r2_secret_key: String,
    pub r2_bucket: String,
    pub jwt_secret: String,
    pub auth_server_uri: String,
}

impl Configuration {
    pub fn gather_configuration() -> Configuration {
        let config = Configuration {
            r2_account_id: env::var("R2_ACCOUNT_ID").unwrap().to_string(),
            r2_access_key: env::var("R2_ACCESS_KEY").unwrap().to_string(),
            r2_secret_key: env::var("R2_SECRET_KEY").unwrap().to_string(),
            r2_bucket: env::var("R2_BUCKET").unwrap().to_string(),
            jwt_secret: env::var("JWT_SECRET").unwrap().to_string(),
            auth_server_uri: env::var("AUTH_SERVER_URI").unwrap().to_string(),
        };

        config
    }
}
