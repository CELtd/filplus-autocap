use std::env;
use anyhow::Result;

/// Central configuration for the allocator runtime.
///
/// Loaded from `.env` file or environment variables.
pub struct AppConfig {
    pub rpc_url: String,
    pub wallet_file: String,
    pub auction_file: String,
    pub registry_file: String,
}

/// Loads application configuration from environment variables.
///
/// Reads `.private/.env` if available. Fails with a descriptive error if any variable is missing.
pub fn load_config() -> Result<AppConfig> {
    // Load from .env file if it exists
    dotenvy::from_filename(".private/.env").ok();

    Ok(AppConfig {
        rpc_url: env::var("RPC_URL")?,
        wallet_file: env::var("WALLET_FILE")?,
        auction_file: env::var("AUCTION_FILE")?,
        registry_file: env::var("REGISTRY_FILE")?,
    })
}
