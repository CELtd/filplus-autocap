mod wallet;
mod rpc;
mod metadata;

use anyhow::Result;
use dotenvy;
use std::env;
use cid::Cid;
use fvm_shared::{ActorID, piece::PaddedPieceSize};
use base64::engine::general_purpose;
use base64::Engine as _;


use wallet::{load_or_create_wallet, Wallet};
use rpc::{send_message_to, fetch_balance, Connection};
use metadata::{serialize_metadata, Metadata};

fn main() -> Result<()> {
    // environment setup
    dotenvy::from_filename(".private/.env").ok(); 
    let rpc_url: String = env::var("RPC_URL")?;
    let wallet_file: String = env::var("WALLET_FILE")?;
    

    // setup connection and load wallet
    let connection: Connection = Connection::new(&rpc_url);
    let wallet: wallet::Wallet = load_or_create_wallet(&wallet_file)?;

    // testnet 
    let testnet_address: String = wallet.address.replacen("f1", "t1", 1);
    println!("📬 filecoin wallet address (testnet format): {}", testnet_address);

    // get fil and datacap balance of wallet
    let balance: String = fetch_balance(&connection, &wallet.address)?;
    println!("💰 fil balance: {} attofil", balance);

    let cid = Cid::try_from("baga6ea4seaqcbzdyshqeqxw2hw2nbv2a45vruq54mc7f3ukgdtqjmdv7n7p7gqq").unwrap();
    let metadata = Metadata {
        provider: 1234 as u64,
        data: cid,
        size: PaddedPieceSize(1 << 10), // 1 GiB
        term_min: 100,
        term_max: 200,
        expiration: 500,
    };
    
    let masterbot_wallet = "f1ypey2rhkbtdassaz5ydtlri3slupzhrylbbguoa";
    let cid = send_message_to(&connection, &wallet, &masterbot_wallet, "1000000000000000", &metadata)?; // 0.001 FIL in atto
    println!("Tx CID: {:?}", cid);
    //let raw = "pmhwcm92aWRlchkE0mRkYXRhWCcBgeIDkiAgIOR4keBIXto9tNDXQOdrGkO8YL5d0UYc4JYOv2/f80Jkc2l6ZRpAAAAAaHRlcm1fbWluGGRodGVybV9tYXgYyGpleHBpcmF0aW9uGQH0";
    //let bytes = base64::engine::general_purpose::STANDARD.decode(raw)?;
    //let metadata: Metadata = serde_cbor::from_slice(&bytes)?;
    //println!("Piece CID: {}", metadata.data.to_string());
    //println!("{:#?}", metadata);
    Ok(())
}