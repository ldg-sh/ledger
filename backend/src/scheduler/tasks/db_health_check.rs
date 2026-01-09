use crate::{context::AppContext, scheduler::ScheduledJob};
use std::sync::Arc;
use std::time::Duration;

pub struct DatabaseHealthCheck;

#[async_trait::async_trait]
impl ScheduledJob for DatabaseHealthCheck {
    fn name(&self) -> &'static str {
        "database_health_check"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(60 * 30)
    }

    async fn run(&self, data: Arc<AppContext>) -> anyhow::Result<()> {
        data.postgres_service
            .ping()
            .await
            .map_err(anyhow::Error::new)?;

        tracing::info!(
            target: "scheduler",
            job = %self.name(),
            "Postgres connection checked"
        );

        Ok(())
    }
}
