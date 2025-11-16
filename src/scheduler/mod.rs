use crate::{context::AppContext, scheduler::manager::Scheduler};
use std::sync::Arc;

pub mod manager;
pub mod tasks;

#[async_trait::async_trait]
pub trait ScheduledJob: Send + Sync {
    fn name(&self) -> &'static str;
    fn interval(&self) -> std::time::Duration;
    async fn run(&self, data: Arc<AppContext>) -> anyhow::Result<()>;
}

pub fn configure_scheduler() -> Scheduler {
    Scheduler::new()
        .register(tasks::storage_health_check::StorageHealthCheck)
        .register(tasks::redis_health_check::RedisHealthCheck)
        .register(tasks::db_health_check::DatabaseHealthCheck)
        .register(tasks::track_files::TrackFiles)
}
