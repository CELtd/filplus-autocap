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
    pub allocator_address_hex: String,
    pub allocator_private_key: String,
    pub metallocator_contract_address: String,
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
        allocator_address_hex: env::var("ALLOCATOR_ADDRESS_HEX")?,
        allocator_private_key: env::var("ALLOCATOR_PRIVATE_KEY")?,
        metallocator_contract_address: env::var("METALLOCATOR_CONTRACT_ADDRESS")?,

    })
}
