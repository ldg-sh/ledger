use anyhow::Result;
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Config, Pool, Runtime};
use redis::{cmd, pipe, Commands, JsonCommands};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;

pub struct RedisService {
    pool: Pool,
}

impl RedisService {
    pub fn new(url: &str) -> Self {
        let pool = Config::from_url(url)
            .create_pool(Some(Runtime::Tokio1))
            .expect("create pool");
        Self { pool }
    }

    pub async fn get(&self, k: &str) -> Result<Option<String>> {
        let mut conn = self.pool.get().await?;
        let v = conn.get(k).await?;
        Ok(v)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, k: &str) -> Result<Option<T>> {
        let mut conn = self.pool.get().await?;

        let raw: Option<String> = cmd("JSON.GET")
            .arg(k)
            .query_async(&mut conn)
            .await?;
        Ok(
            match raw {
                Some(s) => Some(serde_json::from_str::<T>(&s)?),
                None => None
            }
        )
    }

    pub async fn set_ex_json<T: Serialize>(&self, k: &str, v: &T, ttl: u64) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let v = serde_json::to_string(v)?;

        pipe()
            .cmd("JSON.SET").arg(&k).arg("$").arg(v)
            .ignore()
            .expire(&k, ttl as i64)
            .query_async::<()>(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn set_ex(&self, k: &str, v: &str, ttl: u64) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.set_ex(k, v, ttl).await?;
        Ok(())
    }


    pub async fn del(&self, k: &str) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.del(k).await?;
        Ok(())
    }
}
