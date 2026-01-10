use crate::modules::redis::builder::RedisKeyBuilder;
use crate::modules::redis::redis_service::RedisService;
use crate::types::file::RedisFileMeta;
use anyhow::Result;
use redis::AsyncCommands;
use serde_json;

impl RedisService {
    pub async fn store_file_log(&self, file_key: &str, file_data: &RedisFileMeta) -> Result<()> {
        let meta_key = RedisKeyBuilder::file_log_key(file_key);
        let payload = serde_json::to_string(&file_data)?;
        let mut conn = self.connection().await?;

        if let Ok(Some(previous_json)) = conn.get::<_, Option<String>>(&meta_key).await {
            if let Ok(previous) = serde_json::from_str::<RedisFileMeta>(&previous_json) {
                let previous_set = RedisKeyBuilder::generation_members_key(previous.generation);
                let _: () = conn.srem(previous_set, file_key).await?;
            }
        }

        let generation_set = RedisKeyBuilder::generation_members_key(file_data.generation);
        let _: () = redis::pipe()
            .cmd("SET")
            .arg(&meta_key)
            .arg(&payload)
            .ignore()
            .cmd("SADD")
            .arg(&generation_set)
            .arg(file_key)
            .ignore()
            .query_async(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_file_meta(&self, file_key: &str) -> Result<RedisFileMeta> {
        let meta_key = RedisKeyBuilder::file_log_key(file_key);

        let mut conn = self.connection().await?;
        let payload: String = conn.get(&meta_key).await?;
        let file_data: RedisFileMeta = serde_json::from_str(&payload)?;

        Ok(file_data)
    }
}
