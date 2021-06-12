use anyhow::Result;
use crate::config::{QUEUE_NAME, AppState};
use celery::broker::RedisBroker;
use celery::beat::{CronSchedule, DeltaSchedule};
use celery::task::TaskResult;
use tokio::time::Duration;
use crate::tasks::{add, long_running_task, pull};
use substrate_subxt::Runtime;

pub struct Producer;

impl Producer {
    pub async fn start(app_state: &AppState<'_>) -> Result<()> {

        let mut beat = celery::beat!(
                broker = RedisBroker { std::env::var("REDIS_ADDR").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into())},
                tasks = [
                    "add" => {
                        add,
                        schedule = DeltaSchedule::new(Duration::from_secs(5)),
                        args = (1, 2),
                    },
                    "long_running" => {
                        long_running_task,
                        schedule = CronSchedule::from_string("*/2 * * * *")?,
                        args = (Some(1),),
                    },
                    "pull" => {
                        pull,
                        schedule = DeltaSchedule::new(Duration::from_secs(10)),
                        args = (),
                    },
                ],
                task_routes = [
                    "*" => QUEUE_NAME,
                ],
            ).await?;

        beat.start().await?;

        Ok(())
    }
}