use celery::task::TaskResult;
use substrate_subxt::{ClientBuilder};
use redis::ConnectionLike;
use anyhow::{Result, Error};

use crate::runtime::Runtime;
use crate::config::{AppState, REDIS_TIMEOUT, Settings};
use celery::prelude::*;
use std::env;

#[celery::task]
pub(crate) fn add(x: i32, y: i32) -> TaskResult<i32> {
    let res = x + y;
    println!("{:?}", res);
    Ok(res)
}

#[celery::task]
pub(crate) fn long_running_task(secs: Option<u64>) -> TaskResult<()> {
    println!("{:?}", secs);
    unimplemented!()
}

#[celery::task]
pub(crate) async fn pull(settings: &Settings) -> TaskResult<()> {
    let client = ClientBuilder::<Runtime>::new()
        .set_url(std::env::var("CHAIN_RPC_URL").unwrap_or_else(|_| "ws://127.0.0.1:9944".into()))
        .build()
        .await.with_unexpected_err(|| {
        "Chain node server error"
    })?;

    println!("settings: {:?}", settings);


    let finalized_head = client.finalized_head().await.with_unexpected_err(|| {
        "get chain node server finalized head error"
    })?;

    let finalized_block_number = client.block(Some(finalized_head)).await.with_unexpected_err(|| {
        "get chain node server finalized head error"
    })?;
    //TODO: add block decode range

    Ok(())
}
