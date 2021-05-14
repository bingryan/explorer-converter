use anyhow::Result;
use crate::config::{QUEUE_NAME, CELERY_HEARTBEAT};
use celery::broker::RedisBroker;
use celery::beat::{CronSchedule, DeltaSchedule};
use celery::task::TaskResult;
use crate::tasks::{add, long_running_task};

pub struct Consumer;


impl Consumer {
    pub async fn start() -> Result<()> {
        let mut celery = celery::app!(
            broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into())},
            tasks = [
                add,
                long_running_task,
            ],
            task_routes = [
                "*" => QUEUE_NAME,
            ],
            prefetch_count = num_cpus::get() as u16,
            heartbeat = CELERY_HEARTBEAT,
        ).await?;

        celery.display_pretty().await;
        celery.consume_from(&[QUEUE_NAME]).await?;

        Ok(())
    }
}