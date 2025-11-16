use crate::context::AppContext;
use crate::scheduler::ScheduledJob;
use std::sync::Arc;
use tokio::{task, time};

pub struct Scheduler {
    jobs: Vec<Arc<dyn ScheduledJob>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { jobs: vec![] }
    }

    pub fn register<J: ScheduledJob + 'static>(mut self, job: J) -> Self {
        self.jobs.push(Arc::new(job));
        self
    }

    pub async fn start(self, data: Arc<AppContext>) {
        for job in self.jobs {
            let data = data.clone();
            let job = job.clone();
            task::spawn(async move {
                let mut interval = time::interval(job.interval());
                loop {
                    interval.tick().await;
                    if let Err(err) = job.run(data.clone()).await {
                        tracing::error!(target: "scheduler", "Job {} failed: {:?}", job.name(), err);
                    }
                }
            });
        }
    }
}
