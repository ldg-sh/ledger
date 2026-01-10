use crate::{context::AppContext, scheduler::ScheduledJob};
use std::sync::Arc;
use std::time::Duration;

pub struct RedisHealthCheck;

#[async_trait::async_trait]
impl ScheduledJob for RedisHealthCheck {
    fn name(&self) -> &'static str {
        "redis_health_check"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(30 * 60) // 30 minutes
    }

    async fn run(&self, data: Arc<AppContext>) -> anyhow::Result<()> {
        data.redis_service
            .ping()
            .await
            .map_err(anyhow::Error::new)?;

        tracing::info!(
            target: "scheduler",
            job = %self.name(),
            "Redis connection checked"
        );

        Ok(())
    }
}
