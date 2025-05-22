mod wallet;
mod rpc;
mod utils;
mod masterbot;
mod auction;
mod transaction;
mod allocation;
mod registry;

use fvm_ipld_encoding::{from_slice};
use anyhow::Result;
use reqwest::blocking::Client;
use filecoin_signer::key_derive;
use base64::Engine as _;
use base64::engine::general_purpose;
use hex;

use wallet::{load_or_create_wallet};
use rpc::{fetch_balance, send_fil_to, fetch_datacap_balance, get_block_info, get_chain_head_block_number, fetch_datacap_allowance, Connection};
use utils::format_datacap_size_str;
use masterbot::MasterBot;


const WALLET_FILE: &str = ".private/.wallet.json";
const AUCTION_FILE: &str = ".private/.auction.json";
const REGISTRY_FILE: &str = ".private/.registry.json";
const RPC_URL: &str ="https://api.calibration.node.glif.io/rpc/v1?token=Uv8tchy--D9NmPtBzhmMHvRmcIeSWtWHicryEG50UfM=";

fn main() -> Result<()> {
    //let rpc_url="https://api.node.glif.io/rpc/v1";
    let connection= Connection::new(RPC_URL);

    let wallet = load_or_create_wallet(WALLET_FILE)?;
    //let key = key_derive(
    //    &wallet.mnemonic,
    //    &wallet.derivation_path,
    //    "",                // optional password, unused in Filecoin
    //    &wallet.language
    //)?;
    //let hex_encoded = hex::encode(key.private_key.0);
    //println!("🔐 Hex private key: 0x{}", hex_encoded); // prepend 0x for forge

    let testnet_address = wallet.address.replacen("f1", "t1", 1);
    println!("📬 Filecoin wallet address (testnet format): {}", testnet_address);

    let balance = fetch_balance(&connection, &wallet.address)?;

    println!("💰 FIL balance: {} attoFIL", balance);

    //let amount_atto = "1"; // 0.001 FIL in attoFIL
    //let cid = send_to_burn_address(&client, &wallet, rpc_url, amount)?;
    //let cid = send_fil_to(&client, &wallet,"f099", &rpc_url, amount_atto)?;
    //println!("🔥 Sent to burn address. CID: {}", cid);

    //let balance = fetch_balance(&wallet, rpc_url)?;

    let datacap_bytes = fetch_datacap_balance(&connection, &wallet.address)?;
    let readable = format_datacap_size_str(&datacap_bytes);
    println!("✅ Datacap balance: {}", readable);

    //let allowance = fetch_datacap_allowance(&client, &wallet.address, rpc_url)?;
    //let readable = format_datacap_size_str(&allowance);
    //println!("✅ Datacap allowance: {}", readable);

    // Initialize and run masterbot
    let current_block = get_chain_head_block_number(&connection).unwrap_or(0);
    let mut bot = MasterBot::new(wallet, connection, current_block, 5, AUCTION_FILE, REGISTRY_FILE)?;
    
    bot.run();

    //let cid = push_datacap_balance_query(&wallet.address, &wallet.mnemonic, rpc_url)?;

    //let hex_return = "4d0003782dace9d9000000000000"; // <- your actual return value, no '0x'

    //let amount = decode_datacap_return(hex_return)?;
    //println!("✅ Datacap balance: {} attoFIL", amount.atto());
    Ok(())
}