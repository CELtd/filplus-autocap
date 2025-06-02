use crate::{
    wallet::load_or_create_wallet,
    rpc::{Connection, fetch_balance, fetch_datacap_balance, get_chain_head_block_number},
    utils::format_datacap_size_str,
    masterbot::MasterBot,
    runtime::config::AppConfig,
};
use anyhow::Result;

pub fn run_app(config: AppConfig) -> Result<()> {
    let connection = Connection::new(&config.rpc_url);
    let wallet = load_or_create_wallet(&config.wallet_file)?;
    let testnet_address = wallet.address.replacen("f1", "t1", 1);
    log::info!("📬 Filecoin wallet (testnet): {}", testnet_address);

    let balance = fetch_balance(&connection, &wallet.address)?;
    log::info!("💰 FIL balance: {} attoFIL", balance);

    let datacap_bytes = fetch_datacap_balance(&connection, &wallet.address)?;
    log::info!("✅ Datacap balance: {}", format_datacap_size_str(&datacap_bytes));

    let current_block = get_chain_head_block_number(&connection).unwrap_or(0);
    let mut bot = MasterBot::new(wallet, connection, current_block, &config.auction_file, &config.registry_file)?;
    bot.run();

    Ok(())
}
