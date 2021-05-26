use config::{ConfigError, Config, File};
use std::result;
use std::env;
use std::path::PathBuf;
use meilisearch_sdk::client::Client;
use substrate_subxt::{Runtime, ClientBuilder, Client as SubClient};

pub const QUEUE_NAME: &str = "explorer";

pub const CELERY_HEARTBEAT: Option<u16> = Some(10);

pub const CONFIG_FILE: &'static str = "explorer.toml";


pub struct AppState<'a, T: Runtime> {
    pub meili_client: Client<'a>,
    pub sub_client: SubClient<T>,
}


impl<T: Runtime> AppState<'_, T> {
    pub fn new(settings: &Settings) -> AppState<T> {
        let chain_rpc_url = env::var("CHAIN_RPC_URL").ok()
            .unwrap_or_else(|| "ws://127.0.0.1:9944".to_string());

        let sub_client = ClientBuilder::<dyn Runtime>::new()
            .set_url(chain_rpc_url)
            .build()
            .await?;


        AppState {
            meili_client: Client::new(
                &settings.meilisearch.host,
                &settings.meilisearch.apikey,
            ),
            sub_client,
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct MeiliSearch {
    pub host: String,
    pub apikey: String,
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
}

impl Settings {
    pub fn build(file: PathBuf) -> result::Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name(file.into_os_string().into_string().unwrap().as_str()))?;
        s.try_into()
    }
}