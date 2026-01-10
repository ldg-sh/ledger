use crate::{context::AppContext, scheduler::ScheduledJob, types::file::RedisFileMeta};
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use std::time::Duration;

const MAX_GENERATIONS: i64 = 6;

pub struct TrackFiles;

#[async_trait::async_trait]
impl ScheduledJob for TrackFiles {
    fn name(&self) -> &'static str {
        "track_files"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(5)
    }

    async fn run(&self, data: Arc<AppContext>) -> anyhow::Result<()> {
        let existing_cursor = data.redis_service.retrieve_file_scan_cursor().await?;

        // list_files now returns (Vec<TFileInfo>, Option<String>)
        let (files, next_cursor) = data.s3_service.list_files(existing_cursor).await?;

        let current_gen = match data.redis_service.retrieve_current_generation().await? {
            Some(generation_value) => generation_value % MAX_GENERATIONS,
            None => {
                data.redis_service.set_current_generation(&0).await?;
                0
            }
        };

        let next_gen = (current_gen + 1) % MAX_GENERATIONS;

        stream::iter(&files)
            .for_each_concurrent(75, |f| {
                let redis_file = RedisFileMeta {
                    info: f.clone(),
                    generation: current_gen,
                };
                let data = data.clone();
                async move {
                    data.redis_service
                        .store_file_log(&f.key, &redis_file)
                        .await
                        .ok();
                }
            })
            .await;

        // update cursor based on pagination
        match next_cursor {
            Some(cursor) => {
                data.redis_service
                    .store_file_scan_cursor(&cursor)
                    .await
                    .ok();
                tracing::debug!(
                    target: "scheduler",
                    job = %self.name(),
                    count = files.len(),
                    "Page processed — more pages remaining"
                );
            }
            None => {
                data.redis_service.clear_file_scan_cursor().await.ok();

                tracing::debug!("Grabbing current generation...");

                let prune_generation = (current_gen + MAX_GENERATIONS - 1) % MAX_GENERATIONS;
                tracing::debug!("About to prune generation {}", prune_generation);
                data.redis_service
                    .prune_generation(prune_generation)
                    .await?;
                tracing::debug!("Pruned!!!");

                tracing::debug!(
                    target: "scheduler",
                    job = %self.name(),
                    count = files.len(),
                    pruned_generation = prune_generation,
                    "Pruned previous generation"
                );

                data.redis_service.set_current_generation(&next_gen).await?;

                tracing::debug!(
                    target: "scheduler",
                    job = %self.name(),
                    count = files.len(),
                    "Finished full scan — cursor cleared"
                );
            }
        }

        Ok(())
    }
}
