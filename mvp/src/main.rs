mod wallet;
mod rpc;
mod utils;
mod masterbot;
mod auction;
mod transaction;
mod allocation;
mod registry;
mod constants;

use anyhow::Result;
use dotenvy;
use std::env;

use wallet::{load_or_create_wallet};
use rpc::{fetch_balance, fetch_datacap_balance, get_chain_head_block_number, Connection};
use utils::format_datacap_size_str;
use masterbot::MasterBot;

fn main() -> Result<()> {
    // Environment setup
    dotenvy::from_filename(".private/.env").ok(); 
    let RPC_URL: String = env::var("RPC_URL")?;
    let WALLET_FILE: String = env::var("WALLET_FILE")?;
    let AUCTION_FILE: String  = env::var("AUCTION_FILE")?;
    let REGISTRY_FILE: String = env::var("REGISTRY_FILE")?;

    // Setup connection and load wallet
    let connection: Connection = Connection::new(&RPC_URL);
    let wallet: wallet::Wallet = load_or_create_wallet(&WALLET_FILE)?;

    // Testnet 
    let testnet_address: String = wallet.address.replacen("f1", "t1", 1);
    println!("📬 Filecoin wallet address (testnet format): {}", testnet_address);

    // Get FIL and DataCap Balance of wallet
    let balance: String = fetch_balance(&connection, &wallet.address)?;
    println!("💰 FIL balance: {} attoFIL", balance);
    let datacap_bytes: String = fetch_datacap_balance(&connection, &wallet.address)?;

    let readable: String = format_datacap_size_str(&datacap_bytes);
    println!("✅ Datacap balance: {}", readable);

    // Initialize and run masterbot
    let current_block: u64 = get_chain_head_block_number(&connection).unwrap_or(0);
    let mut bot: MasterBot = MasterBot::new(wallet, connection, current_block, &AUCTION_FILE, &REGISTRY_FILE)?;
    
    bot.run();

    Ok(())
}