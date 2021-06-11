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
    println!("pull 1101");

    let chain_url = env::var("CHAIN_RPC_URL").ok()
        .unwrap_or_else(|| (&settings.chain.rpc_url).to_string());

    let client = ClientBuilder::<Runtime>::new()
        .set_url(chain_url)
        .build()
        .await.with_unexpected_err(|| {
        "Chain node server error"
    })?;

    let block_number = 1;

    let block_hash = client.block_hash(Some(block_number.into())).await
        .with_unexpected_err(|| {
            "Chain node get block hash failed"
        })?;

    if let Some(hash) = block_hash {
        println!("Block hash for block number {}: {}", block_number, hash);
    } else {
        println!("Block number {} not found.", block_number);
    }

    Ok(())
}
