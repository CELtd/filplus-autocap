use serde_json::Value;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;

use crate::metadata::{Metadata};
use crate::transaction::{Transaction};
use crate::utils::wei_to_fil;

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

