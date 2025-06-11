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
    // Establish RPC connection
    let connection = Connection::new(&config.rpc_url);

    // Load or create the wallet
    let wallet = load_or_create_wallet(&config.wallet_file)?;

    // Print wallet address in testnet format
    let testnet_address = wallet.address.replacen("f1", "t1", 1);
    log::info!("📬 Filecoin wallet (testnet): {}", testnet_address);

    // Fetch and log wallet FIL balance
    let balance = fetch_balance(&connection, &wallet.address)?;
    log::info!("💰 FIL balance: {} attoFIL", balance);

    // Fetch and log wallet DataCap balance
    let datacap = fetch_datacap_balance(&connection, &wallet.address).unwrap_or(0);
    log::info!("✅ Datacap balance: {}", format_datacap_size(&datacap));

    // Fetch latest chain head block number
    let current_block = get_chain_head_block_number(&connection).unwrap_or(0);

    // Initialize the MasterBot with loaded state
    let mut bot = MasterBot::new(
        wallet,
        connection,
        current_block,
        &config.auction_file,
        &config.registry_file,
    )?;

    // Start the masterbot auction loops
    bot.run();

    Ok(())
}
