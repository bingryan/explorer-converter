#![allow(unused_variables)]

mod cli;
mod cmd;
pub mod config;
mod tasks;
mod runtime;
mod log;
mod filter;
mod scheduler;

use anyhow::Result;
use env_logger::Env;
use std::env;
use tokio::time::Duration;
use crate::cmd::producer::Producer;
use crate::cmd::consumer::Consumer;
use crate::config::{QUEUE_NAME, CELERY_HEARTBEAT, CONFIG_FILE, Settings, ExplorerLog, AppState};




#[tokio::main]
async fn main() -> Result<()> {

    dotenv::dotenv().ok();

    let config_file = env::current_dir()?.join(CONFIG_FILE);

    let settings = Settings::build(config_file).unwrap();


    ExplorerLog::init(&settings);

    let bind_address = &settings.server.bind_address;
    let state = AppState::new(&settings);

    let meili_client = state.meili_client;
    let meili_client_state = meili_client.is_healthy().await;
    if !meili_client_state {
        llog::error!("Could not ping meilisearch server to address {} with apikey: {}",
                     &settings.meilisearch.host,
                     &settings.meilisearch.apikey);
        std::process::exit(101);
    }

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
        llog::error!("Error: {}", e);
        std::process::exit(101);
    }
    Ok(())
}