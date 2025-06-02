use std::env;
use anyhow::Result;

pub struct AppConfig {
    pub rpc_url: String,
    pub wallet_file: String,
    pub auction_file: String,
    pub registry_file: String,
}

pub fn load_config() -> Result<AppConfig> {
    dotenvy::from_filename(".private/.env").ok();

    Ok(AppConfig {
        rpc_url: env::var("RPC_URL")?,
        wallet_file: env::var("WALLET_FILE")?,
        auction_file: env::var("AUCTION_FILE")?,
        registry_file: env::var("REGISTRY_FILE")?,
    })
}
