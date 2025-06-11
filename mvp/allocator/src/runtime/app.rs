use crate::{
    wallet::load_or_create_wallet,
    rpc::{Connection, fetch_balance, fetch_datacap_balance, get_chain_head_block_number},
    utils::format_datacap_size,
    masterbot::MasterBot,
    runtime::config::AppConfig,
};
use anyhow::Result;

/// Main application runner.
/// 
/// Initializes the connection, wallet, logs balances, and launches the MasterBot loop.
pub fn run_app(config: AppConfig) -> Result<()> {

    // Initialize the MasterBot with loaded state
    let mut bot = MasterBot::new(config)?;

    // Start the masterbot auction loops
    bot.run();

    Ok(())
}
