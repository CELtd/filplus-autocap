use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;


/// Represents the current auction state.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auction {
    pub block_number: u64,
    pub transactions: Vec<Transaction>,

    #[serde(skip)] // don't serialize this field
    pub file_path: String,
}


impl Auction {
    pub fn new(start_block: u64, file_path: &str) -> Self {
        Auction {
            block_number: start_block,
            transactions: vec![],
            file_path: file_path.to_string(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn load_or_new(file_path: &str) -> Result<Self> {
        if let Ok(contents) = std::fs::read_to_string(file_path) {
            let mut auction: Auction = serde_json::from_str(&contents)?;
            auction.file_path = file_path.to_string();
            Ok(auction)
        } else {
            Ok(Auction::new(0, file_path))
        }
    }

    pub fn reset(&mut self) {
        self.transactions.clear();
        self.save();
    }
}

/// Represents the reward obtained from an auction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuctionReward {
    pub address: String,
    pub reward: u64,
}
impl AuctionReward {
    pub fn new(address: String, reward: u64) -> Self {
        AuctionReward {
            address: address,
            reward: reward,
        }
    }
}