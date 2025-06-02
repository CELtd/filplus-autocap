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

