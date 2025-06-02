use crate::transaction::{Transaction, TransactionDisplay};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::fs;

/// Internal auction state, used in logic.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auction {
    pub block_number: u64,
    pub transactions: Vec<Transaction>,

    #[serde(skip)] // Do not include in serialized JSON
    pub file_path: String,
}

impl Auction {
    /// Create a new auction instance starting at a block.
    pub fn new(start_block: u64, file_path: &str) -> Self {
        Auction {
            block_number: start_block,
            transactions: vec![],
            file_path: file_path.to_string(),
        }
    }

    /// Save the auction state to file using human-readable format.
    pub fn save(&self) -> Result<()> {
        let display: AuctionDisplay = self.into(); // Convert to pretty format
        let json = serde_json::to_string_pretty(&display)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    /// Load from file or create new one if not found.
    pub fn load_or_new(file_path: &str) -> Result<Self> {
        if let Ok(contents) = fs::read_to_string(file_path) {
            let mut auction: Auction = serde_json::from_str(&contents)?;
            auction.file_path = file_path.to_string();
            Ok(auction)
        } else {
            Ok(Auction::new(0, file_path))
        }
    }

    /// Reset the auction by clearing all transactions and saving.
    pub fn reset(&mut self) {
        self.transactions.clear();
        let _ = self.save(); // Best-effort save
    }
}

/// A display-friendly version of Auction, for serialization purposes only.
#[derive(Debug, Serialize)]
pub struct AuctionDisplay {
    pub block_number: u64,
    pub transactions: Vec<TransactionDisplay>,
}

impl From<&Auction> for AuctionDisplay {
    fn from(a: &Auction) -> Self {
        AuctionDisplay {
            block_number: a.block_number,
            transactions: a.transactions.iter().map(TransactionDisplay::from).collect(),
        }
    }
}

/// Represents the reward obtained from an auction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuctionReward {
    pub address: String,
    pub reward: u64,
}

impl AuctionReward {
    pub fn new(address: String, reward: u64) -> Self {
        AuctionReward { address, reward }
    }
}
