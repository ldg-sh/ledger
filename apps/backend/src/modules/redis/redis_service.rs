use chrono::Duration;
use deadpool_redis::{Config as RedisPoolConfig, Connection, Pool, Runtime};
use redis::{self, AsyncCommands, ErrorKind, RedisError, RedisResult};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use std::fmt::Display;

pub struct RedisService {
    pool: Pool,
}

impl RedisService {
    pub async fn new(url: &str) -> Result<Self, RedisError> {
        let pool = RedisPoolConfig::from_url(url)
            .create_pool(Some(Runtime::Tokio1))
            .map_err(|err| map_pool_error("failed to create redis pool", err))?;

        {
            let mut conn = pool
                .get()
                .await
                .map_err(|err| map_pool_error("failed to get redis connection", err))?;

            redis::cmd("PING").query_async::<_, ()>(&mut conn).await?;
        }

        Ok(Self { pool })
    }

    pub async fn ping(&self) -> Result<(), RedisError> {
        let mut conn = self.connection().await?;
        redis::cmd("PING").query_async::<_, ()>(&mut conn).await
    }

    pub(crate) async fn connection(&self) -> Result<Connection, RedisError> {
        self.pool
            .get()
            .await
            .map_err(|err| map_pool_error("failed to get redis connection", err))
    }

    pub async fn set_value(
        &self,
        key: &str,
        value: &str,
        ttl: Option<Duration>,
    ) -> RedisResult<()> {
        let mut conn = self.connection().await?;
        if let Some(ttl) = ttl {
            let seconds = ttl.num_seconds().max(0) as u64;
            let _: () = conn.set_ex(key, value, seconds).await?;
        } else {
            let _: () = conn.set(key, value).await?;
        }
        Ok(())
    }

    pub async fn get_value(&self, key: &str) -> RedisResult<Option<String>> {
        let mut conn = self.connection().await?;
        conn.get(key).await
    }

    pub async fn remove_value(&self, key: &str) -> RedisResult<bool> {
        let mut conn = self.connection().await?;
        let deleted: u64 = conn.del(key).await?;
        Ok(deleted > 0)
    }

    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T) -> redis::RedisResult<()> {
        let mut conn = self.connection().await?;
        let json = serde_json::to_string(value).map_err(|err| {
            RedisError::from((
                ErrorKind::TypeError,
                "failed to serialize value to json",
                err.to_string(),
            ))
        })?;
        let _: () = conn.set(key, json).await?;
        Ok(())
    }

    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> redis::RedisResult<Option<T>> {
        let mut conn = self.connection().await?;
        let data: Option<String> = conn.get(key).await?;
        Ok(match data {
            Some(s) => Some(serde_json::from_str::<T>(&s).map_err(|err| {
                RedisError::from((
                    ErrorKind::TypeError,
                    "failed to deserialize json",
                    err.to_string(),
                ))
            })?),
            None => None,
        })
    }
}

fn map_pool_error(err_msg: &'static str, err: impl Display) -> RedisError {
    RedisError::from((ErrorKind::IoError, err_msg, err.to_string()))
}
