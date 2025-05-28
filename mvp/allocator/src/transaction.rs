use serde::{Serialize, Deserialize};
use serde_json::Value;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;

use crate::metadata::{Metadata, MetadataDisplay};

/// Internal representation of a transaction with optional decoded metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub cid: String,
    pub from: String,
    pub to: String,
    pub value_fil: f64,
    pub block_number: u64,
    pub metadata: Option<Metadata>, // Parsed CBOR metadata (raw form)
}

/// Human-friendly version of a transaction for output/logging.
#[derive(Debug, Serialize)]
pub struct TransactionDisplay {
    pub cid: String,
    pub from: String,
    pub to: String,
    pub value_fil: f64,
    pub block_number: u64,
    pub metadata: Option<MetadataDisplay>, // Pretty metadata (CID as string)
}

impl From<&Transaction> for TransactionDisplay {
    fn from(tx: &Transaction) -> Self {
        TransactionDisplay {
            cid: tx.cid.clone(),
            from: tx.from.clone(),
            to: tx.to.clone(),
            value_fil: tx.value_fil,
            block_number: tx.block_number,
            metadata: tx.metadata.as_ref().map(MetadataDisplay::from),
        }
    }
}

/// Filters incoming messages to the given wallet address, extracts value + metadata.
pub fn filter_incoming_txs(messages: &Value, wallet_address: &str, block_number: u64) -> Vec<Transaction> {
    let mut txs = Vec::new();

    if let Some(array) = messages.as_array() {
        for item in array {
            let message = &item["Message"];
            let to = message["To"].as_str().unwrap_or("").to_lowercase();

            if to == wallet_address.replacen("f1", "t1", 1).to_lowercase() {
                // Attempt to decode embedded CBOR metadata from base64 Params
                let metadata = message.get("Params").and_then(|param| {
                    let raw = param.as_str()?;
                    let bytes = base64_engine.decode(raw).ok()?;
                    match serde_cbor::from_slice::<Metadata>(&bytes) {
                        Ok(m) => {
                            Some(m)
                        },
                        Err(e) => {
                            println!("❌ CBOR decode error of tx metadata: {:?}", e);
                            None
                        }
                    }
                });

                let tx = Transaction {
                    cid: item["Cid"]["/"].as_str().unwrap_or("").to_string(),
                    from: message["From"].as_str().unwrap_or("").to_string(),
                    to: message["To"].as_str().unwrap_or("").to_string(),
                    value_fil: wei_to_fil(message["Value"].as_str().unwrap_or("0")),
                    block_number,
                    metadata,
                };

                txs.push(tx);
            }
        }
    }

    txs
}

/// Converts attoFIL (string) to FIL (float).
fn wei_to_fil(wei: &str) -> f64 {
    wei.parse::<f64>().unwrap_or(0.0) / 1e18
}
