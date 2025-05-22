use serde::{Serialize, Deserialize};
use serde_json::Value;


/// Simplified transaction info tracked for auction logic.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub cid: String,
    pub from: String,
    pub to: String,
    pub value_fil: f64,
    pub block_number: u64,
}
pub fn filter_incoming_txs(messages: &Value, wallet_address: &str, block_number: u64) -> Vec<Transaction> {
    let mut txs = Vec::new();

    if let Some(array) = messages.as_array() {
        for item in array {
            let message = &item["Message"];
            let to = message["To"].as_str().unwrap_or("").to_lowercase();

            if to == wallet_address.replacen("f1", "t1", 1).to_lowercase() {
                let tx = Transaction {
                    cid: item["Cid"]["/"].as_str().unwrap_or("").to_string(),
                    from: message["From"].as_str().unwrap_or("").to_string(),
                    to: message["To"].as_str().unwrap_or("").to_string(),
                    value_fil: wei_to_fil(message["Value"].as_str().unwrap_or("0")),
                    block_number,
                };
                txs.push(tx);
            }
        }
    }

    txs
}

fn wei_to_fil(wei: &str) -> f64 {
    wei.parse::<f64>().unwrap_or(0.0) / 1e18
}