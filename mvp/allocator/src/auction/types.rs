use crate::transaction::{Transaction, TransactionDisplay};
use serde::{Deserialize, Serialize};

/// Internal auction state, used in logic.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Auction {
    pub block_number: u64,
    pub transactions: Vec<Transaction>,

    #[serde(skip)] // Do not include in serialized JSON
    pub file_path: String,
}

impl From<AuctionDisplay> for Auction {
    fn from(display: AuctionDisplay) -> Self {
        Auction {
            block_number: display.block_number,
            transactions: display
                .transactions
                .into_iter()
                .map(Transaction::from)
                .collect(),
            file_path: String::new(), // To be filled in later manually
        }
    }
}


/// A display-friendly version of Auction, for serialization purposes only.
#[derive(Debug, Serialize, Deserialize)]
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
