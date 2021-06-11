use anyhow::Result;
use config::{ConfigError, Config, File};
use std::result;
use std::env;
use std::path::PathBuf;
use redis::Client as RedisClient;
use meilisearch_sdk::client::Client;
use substrate_subxt::{Runtime, ClientBuilder, Client as SubClient};
use std::time::Duration;

pub const REDIS_TIMEOUT: Duration = Duration::from_secs(3);

pub const QUEUE_NAME: &'static str = "explorer";

pub const CELERY_HEARTBEAT: Option<u16> = Some(10);

pub const CONFIG_FILE: &'static str = "explorer.toml";


pub struct AppState<'a> {
    pub meili_client: Client<'a>,
    pub redis_client: RedisClient,
    pub settings: Settings,
}


impl AppState<'_> {
    pub async fn new(settings: &Settings) -> Result<AppState<'_>> {
        let chain_rpc_url = env::var("CHAIN_RPC_URL").ok()
            .unwrap_or_else(|| (&settings.chain.rpc_url).to_string());

        let redis_url = env::var("REDIS_URL").ok()
            .unwrap_or_else(|| (&settings.redis.url).to_string());

        let redis_client = redis::Client::open(redis_url)?;

        Ok(AppState {
            meili_client: Client::new(
                &settings.meilisearch.host,
                &settings.meilisearch.apikey,
            ),
            redis_client,
            settings: settings.to_owned(),
        })
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct MeiliSearch {
    pub host: String,
    pub apikey: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Chain {
    pub rpc_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Redis {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExplorerLog {
    pub log_dir: String,
    pub log_cup: usize,
    pub temp_size: String,
    pub zip_compress: bool,
    pub rolling_type: String,
    pub level: String,
    pub debug: bool,
    pub max_size: String,
    pub file_name: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub log: ExplorerLog,
    pub meilisearch: MeiliSearch,
    pub chain: Chain,
    pub redis: Redis,
}

impl Settings {
    pub fn build(file: PathBuf) -> result::Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(file.into_os_string().into_string().unwrap().as_str()))?;
        s.try_into()
    }
}