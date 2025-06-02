use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;


/// Represents the Registry of datacap credits of different SPs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registry {
    pub block_number: u64,
    pub credits: HashMap<String, u64>,

    #[serde(skip)] // don't serialize this field
    pub file_path: String,
}


impl Registry {
    pub fn new(start_block: u64, file_path: &str) -> Self {
        Registry {
            block_number: start_block,
            credits: HashMap::new(),
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
            let mut auction: Registry = serde_json::from_str(&contents)?;
            auction.file_path = file_path.to_string();
            Ok(auction)
        } else {
            Ok(Registry::new(0, file_path))
        }
    }
}

