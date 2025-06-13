use filecoin_signer::{key_generate_mnemonic, key_derive};
use std::fs;
use std::path::Path;
use anyhow::Result;

use crate::wallet::Wallet;

// Default values for wallet creation
const DERIVATION_PATH: &str = "m/44'/461'/0/0/0"; // Standard path for Filecoin
const LANGUAGE: &str = "en";                      // Mnemonic language (BIP39), defaulting to English

/// Loads an existing wallet from a file, or creates a new one if the file doesn't exist.
/// If creating a new wallet, a fresh mnemonic will be generated and printed to stdout.
/// 
/// # Arguments
/// * `path` - Path to the wallet JSON file.
///
/// # Returns
/// * `Wallet` instance loaded from or written to the file.
pub fn load_or_create_wallet(path: &str) -> Result<Wallet> {
    if Path::new(path).exists() {
        // If wallet file exists, read and deserialize it.
        let data = fs::read_to_string(path)?;
        let wallet: Wallet = serde_json::from_str(&data)?;
        Ok(wallet)
    } else {
        // Create a new wallet if it doesn't exist
        let mnemonic = key_generate_mnemonic()?; // Generate new 12-word phrase
        println!("🧠 Save this mnemonic securely: {}", mnemonic.0); // Prompt user to back it up

        // Derive key pair and address from mnemonic
        let key = key_derive(&mnemonic.0, DERIVATION_PATH, "", LANGUAGE)?;

        // Construct the wallet object
        let wallet = Wallet {
            mnemonic: mnemonic.0,
            address: key.address,
            derivation_path: DERIVATION_PATH.to_string(),
            language: LANGUAGE.to_string(),
        };

        // Save it to disk for persistence
        fs::write(path, serde_json::to_string_pretty(&wallet)?)?;
        Ok(wallet)
    }
}
