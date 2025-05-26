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
use rpc::{fetch_balance, fetch_datacap_balance, get_chain_head_block_number, create_datacap_allocation, Connection};
use utils::format_datacap_size_str;
use allocation::craft_transfer_from_payload;
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
    
    // Test
    let provider_address = "t1v3thkeow3is5ir6zzxylifql74t77bk3xqjr76y".replacen("t1","f1", 1);
    let provider_id = rpc::resolve_id_address(&connection, &provider_address)?;
    println!("{}", provider_id);
    let transfer_params = craft_transfer_from_payload(
                    &provider_id.to_string(), // SP address
                    "bafy2bzacec7a6itfsidhsg3jrdjumrrfmmekvuz3e2n7zwllc5c5dxts7tntw",    // piece CID
                    1048576,           // 1 KiB
                    current_block, //Current block
                    "1048576000000000000000000"          // datacap amount (in bytes) 1 KiB
                )?;
    println!("{:?}", transfer_params);
    let cid = create_datacap_allocation(transfer_params, &connection, &wallet)?;
    println!("Tx CID: {:?}", cid);

    //let mut bot: MasterBot = MasterBot::new(wallet, connection, current_block, &AUCTION_FILE, &REGISTRY_FILE)?;
    
    //bot.run();

    Ok(())
}