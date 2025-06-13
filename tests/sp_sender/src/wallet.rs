use filecoin_signer::{key_generate_mnemonic, key_derive};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

const DERIVATION_PATH: &str = "m/44'/461'/0/0/0";
const LANGUAGE: &str = "en";

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub mnemonic: String,
    pub address: String,
    pub derivation_path: String,
    pub language: String,
}

pub fn load_or_create_wallet(path: &str) -> Result<Wallet> {
    if Path::new(path).exists() {
        let data = fs::read_to_string(path)?;
        let wallet: Wallet = serde_json::from_str(&data)?;
        Ok(wallet)
    } else {
        let mnemonic = key_generate_mnemonic()?;
        println!("🧠 Save this mnemonic securely: {}", mnemonic.0);

        let key = key_derive(&mnemonic.0, DERIVATION_PATH, "", LANGUAGE)?;
        let wallet = Wallet {
            mnemonic: mnemonic.0,
            address: key.address,
            derivation_path: DERIVATION_PATH.to_string(),
            language: LANGUAGE.to_string(),
        };
        fs::write(path, serde_json::to_string_pretty(&wallet)?)?;
        Ok(wallet)
    }
}
