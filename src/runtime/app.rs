use crate::{
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
