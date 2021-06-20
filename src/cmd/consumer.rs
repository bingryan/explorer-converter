use anyhow::Result;
use crate::config::{QUEUE_NAME, CELERY_HEARTBEAT, AppState};
use celery::broker::RedisBroker;
use celery::beat::{CronSchedule, DeltaSchedule};
use celery::task::TaskResult;
use crate::tasks::{add, long_running_task, pull};
use substrate_subxt::Runtime;

pub struct Consumer;


impl Consumer {
    pub async fn start(app_state: &AppState<'_>) -> Result<()> {
        let mut celery = celery::app!(
            broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into())},
            tasks = [
                add,
                long_running_task,
                pull,
            ],
            task_routes = [
                "*" => QUEUE_NAME,
            ],
            // prefetch_count would be either 100 x NUM_CPUS for IO-bound tasks or 2 * NUM_CPUS for CPU-bound tasks.
            prefetch_count = 2 * num_cpus::get() as u16,
            heartbeat = CELERY_HEARTBEAT,
        ).await?;

        celery.display_pretty().await;
        celery.consume_from(&[QUEUE_NAME]).await?;

        Ok(())
    }
}