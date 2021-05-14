#![allow(unused_variables)]

mod cli;
mod cmd;
pub mod config;
mod tasks;

use anyhow::Result;
use env_logger::Env;
use tokio::time::Duration;
use crate::cmd::producer::Producer;
use crate::cmd::consumer::Consumer;
use crate::config::{QUEUE_NAME,CELERY_HEARTBEAT};




#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let matches = cli::build_cli().get_matches();


    let res = match matches.subcommand() {
        ("producer", Some(matches)) => {
            Producer::start().await
        }
        ("consumer", Some(matches)) => {
            Consumer::start().await
        }
        _ => unreachable!(),
    };
    if let Err(e) = res {
        log::error!("Error: {}", e);
        std::process::exit(101);
    }
    Ok(())
}