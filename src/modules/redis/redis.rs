use deadpool_redis::{Config, Pool, Runtime};
use deadpool_redis::redis::AsyncCommands;
use anyhow::Result;

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
