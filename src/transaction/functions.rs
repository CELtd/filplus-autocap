use serde_json::Value;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine;
use log::{info, warn, error};

use crate::metadata::{Metadata};
use crate::transaction::{Transaction};
use crate::utils::wei_to_fil;
use crate::rpc::{Connection,check_msg_success};

/// Filters incoming messages to the given wallet address, extracts value + metadata.
pub fn filter_incoming_txs(messages: &Value, wallet_address: &str, block_number: u64, connection: &Connection) -> Vec<Transaction> {
    let mut txs = Vec::new();

    if let Some(array) = messages.as_array() {
        for item in array {
            let cid_str = item["Cid"]["/"].as_str().unwrap_or("");
            let message = &item["Message"];
            let to = message["To"].as_str().unwrap_or("").to_lowercase();

            if to == wallet_address.replacen("f1", "t1", 1).to_lowercase() {
                // ✅ Check that tx succeeded (e.g., via `StateReplay`)
                match check_msg_success(connection, cid_str) {
                    Ok(true) => {} // continue
                    Ok(false) => {
                        warn!("❌ Tx {} did not succeed (exit code != 0). Skipping.", cid_str);
                        continue;
                    }
                    Err(e) => {
                        warn!("⚠️ Could not verify success of tx {}: {}", cid_str, e);
                        continue;
                    }
                }

                // Decode base64
                let raw = match message.get("Params").and_then(|param| param.as_str()) {
                    Some(s) => s,
                    None => continue,
                };
                let bytes = match base64_engine.decode(raw) {
                    Ok(b) => b,
                    Err(e) => {
                        warn!("❌ Tx {}, Base64 decode error: {:?}", cid_str, e);
                        continue;
                    }
                };
                let metadata = match serde_cbor::from_slice::<Metadata>(&bytes) {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("❌ Tx {}, CBOR decode error: {:?}", cid_str, e);
                        continue;
                    }
                };

                txs.push(Transaction {
                    cid: cid_str.to_string(),
                    from: message["From"].as_str().unwrap_or("").to_string(),
                    to: message["To"].as_str().unwrap_or("").to_string(),
                    value_fil: wei_to_fil(message["Value"].as_str().unwrap_or("0")),
                    block_number,
                    metadata: Some(metadata),
                });
            }
        }
    }

    txs
}
