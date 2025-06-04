use anyhow::Result;
use dotenvy;
use std::env;
use cid::Cid;
use fvm_shared::piece::PaddedPieceSize;
use base64::{engine::general_purpose, Engine as _};
use filecoin_signer::{key_derive};
use base64::encode;

use crate::wallet::{load_or_create_wallet, Wallet};
use crate::rpc::{fetch_balance, send_metadata_tx, import_wallet_key, Connection};
use crate::metadata::Metadata;

pub fn run(connection: &Connection) -> Result<()> {
    dotenvy::from_filename(".private/.env").ok();
    let masterbot_address = env::var("MASTERBOT_ADDRESS")?;
    let wallet_file: String = env::var("WALLET_FILE")?;

    // Load or create wallet
    let wallet: Wallet = load_or_create_wallet(&wallet_file)?;
    let testnet_address = wallet.address.replacen("f1", "t1", 1);
    println!("\u{1F4EC} filecoin wallet address (testnet format): {}", testnet_address);

    // Check FIL balance
    let balance = fetch_balance(connection, &wallet.address)?;
    println!("\u{1F4B0} fil balance: {} attofil", balance);

    // Derive private key and import into Lotus
    let key = key_derive(&wallet.mnemonic, &wallet.derivation_path, "", &wallet.language)?;
    let private_key_base64 = encode(&key.private_key.0);
    let import_result = import_wallet_key(connection, &private_key_base64)?;
    println!("\u{1F4E5} Wallet import response: {}", import_result);

    // Prepare dummy metadata
    let cid = Cid::try_from("baga6ea4seaqcbzdyshqeqxw2hw2nbv2a45vruq54mc7f3ukgdtqjmdv7n7p7gqq")?;
    let metadata = Metadata {
        provider: 1000,
        data: cid,
        size: PaddedPieceSize(1 << 10),
        term_min: 100,
        term_max: 200,
        expiration: 500,
    };

    let cbor_bytes = serde_cbor::to_vec(&metadata)?;
    let hex_encoded = hex::encode(cbor_bytes);
    println!("CBOR hex: {}", hex_encoded);

    // Send metadata + FIL
    let amount_in_fil = 0.001;
    let cid = send_metadata_tx(connection, &wallet, &masterbot_address, amount_in_fil, &metadata)?;
    println!("\u{2705} Sent transaction with metadata and {} FIL. CID: {}", amount_in_fil, cid);

    Ok(())
}

