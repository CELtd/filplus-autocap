use std::fs;
use anyhow::Result;
use serde_json;

use crate::auction::{Auction, AuctionDisplay};

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
        // Save displayable auction - CID as string
        let display: AuctionDisplay = self.into(); // Convert to pretty format
        let json = serde_json::to_string_pretty(&display)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    /// Load from file or create new one if not found.
    pub fn load_or_new(file_path: &str) -> Result<Self> {
        if let Ok(contents) = fs::read_to_string(file_path) {
            let display: AuctionDisplay = serde_json::from_str(&contents)?;
            let mut auction: Auction = display.into();
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
