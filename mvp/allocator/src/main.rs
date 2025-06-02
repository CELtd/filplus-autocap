mod wallet;
mod rpc;
mod utils;
mod masterbot;
mod auction;
mod transaction;
mod allocation;
mod registry;
mod constants;
mod metadata;

use anyhow::Result;
use dotenvy;
use std::env;
use env_logger;
use log;
use flexi_logger::{Logger, Duplicate, FileSpec};

use wallet::{load_or_create_wallet};
use rpc::{fetch_balance, fetch_datacap_balance, get_chain_head_block_number, Connection};
use utils::format_datacap_size_str;
use masterbot::MasterBot;

fn main() -> Result<()> {

    // Environment setup
    dotenvy::from_filename(".private/.env").ok(); 
    let rpc_url: String = env::var("RPC_URL")?;
    let wallet_file: String = env::var("WALLET_FILE")?;
    let auction_file: String  = env::var("AUCTION_FILE")?;
    let registry_file: String = env::var("REGISTRY_FILE")?;

    // Setup connection and load wallet
    let connection: Connection = Connection::new(&rpc_url);
    let wallet: wallet::Wallet = load_or_create_wallet(&wallet_file)?;

    // Testnet 
    let testnet_address: String = wallet.address.replacen("f1", "t1", 1);
    //println!("📬 Filecoin wallet address (testnet format): {}", testnet_address);
    Logger::try_with_str("info")?
    .duplicate_to_stdout(Duplicate::Info)
    .log_to_file(FileSpec::default().directory("logs"))
    .rotate(
        flexi_logger::Criterion::Size(10_000_000),
        flexi_logger::Naming::Numbers,
        flexi_logger::Cleanup::KeepLogFiles(5),
    )
    .start()?;
    log::info!("📬 Filecoin wallet address (testnet format): {}", testnet_address);

    // Get FIL and DataCap Balance of wallet
    let balance: String = fetch_balance(&connection, &wallet.address)?;
    println!("💰 FIL balance: {} attoFIL", balance);
    let datacap_bytes: String = fetch_datacap_balance(&connection, &wallet.address)?;

    let readable: String = format_datacap_size_str(&datacap_bytes);
    println!("✅ Datacap balance: {}", readable);

    // Initialize and run masterbot
    let current_block: u64 = get_chain_head_block_number(&connection).unwrap_or(0);
    
    // Initialize the masterbot
    let mut bot: MasterBot = MasterBot::new(wallet, connection, current_block, &auction_file, &registry_file)?;

    // Run the masterbot 
    bot.run();

    Ok(())
}