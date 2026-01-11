use anyhow::Result;
use chrono::Duration;
use redis::AsyncCommands;

use crate::modules::redis::builder::RedisKeyBuilder;
use crate::modules::redis::redis_service::RedisService;

impl RedisService {
    pub async fn store_file_scan_cursor(&self, cursor: &str) -> Result<()> {
        let key = RedisKeyBuilder::scan_token();

        self.set_value(&key, cursor, Some(Duration::seconds(60))).await?;

        Ok(())
    }

    pub async fn retrieve_file_scan_cursor(&self) -> Result<Option<String>> {
        let key = RedisKeyBuilder::scan_token();

        let cursor_opt = self.get_value(&key).await?;

        Ok(cursor_opt.filter(|s| !s.is_empty()))
    }

    pub async fn clear_file_scan_cursor(&self) -> Result<()> {
        let key = RedisKeyBuilder::scan_token();

        self.remove_value(&key).await?;

        Ok(())
    }

    pub async fn set_current_generation(&self, generation: &i64) -> Result<()> {
        let key = RedisKeyBuilder::generation_key();

        self.set_value(&key, &generation.to_string(), None).await?;

        Ok(())
    }

    pub async fn retrieve_current_generation(&self) -> Result<Option<i64>> {
        let key = RedisKeyBuilder::generation_key();

        let gen_opt = self.get_value(&key).await?;

        Ok(gen_opt.map(|s| s.parse::<i64>().unwrap()))
    }

    pub async fn prune_generation(&self, generation: i64) -> anyhow::Result<()> {
        let mut conn = self.connection().await?;
        let set_key = RedisKeyBuilder::generation_members_key(generation);
        let members: Vec<String> = conn.smembers(&set_key).await?;

        if members.is_empty() {
            let _: () = conn.del(&set_key).await?;
            return Ok(());
        }

        let mut pipeline = redis::pipe();
        for member in &members {
            pipeline
                .del(RedisKeyBuilder::file_log_key(member));
        }
        pipeline.cmd("DEL").arg(&set_key);
        let _: () = pipeline.query_async(&mut conn).await?;

        Ok(())
    }
}
