use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub mnemonic: String,
    pub address: String,
    pub derivation_path: String,
    pub language: String,
}
