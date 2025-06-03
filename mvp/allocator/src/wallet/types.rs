use serde::{Deserialize, Serialize};

/// Represents a Filecoin wallet.
/// This struct stores everything needed to derive the key and sign transactions.
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    /// BIP39 mnemonic phrase used to derive the private key.
    pub mnemonic: String,

    /// Public Filecoin address (e.g., f1... or t1... for testnet).
    pub address: String,

    /// Derivation path used for key derivation (usually "m/44'/461'/0/0/0").
    pub derivation_path: String,

    /// Language used for the mnemonic phrase (usually "english").
    pub language: String,
}
