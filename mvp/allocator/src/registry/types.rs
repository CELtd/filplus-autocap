use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

/// Tracks how much datacap credit each storage provider (SP) has accumulated.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registry {
    /// The last block number at which the registry was updated.
    pub block_number: u64,

    /// A mapping from SP address (as a string) to datacap credit in bytes.
    pub credits: HashMap<String, u64>,

    /// Local file path where this registry is stored.
    /// Not serialized when saving to JSON.
    #[serde(skip)]
    pub file_path: String,
}

impl Registry {
    /// Creates a new registry starting at a given block.
    pub fn new(start_block: u64, file_path: &str) -> Self {
        Registry {
            block_number: start_block,
            credits: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }

    /// Saves the registry state to the specified file in human-readable JSON.
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&self.file_path, json)?;
        Ok(())
    }

    /// Loads the registry from a file or creates a new one if it doesn't exist.
    pub fn load_or_new(file_path: &str) -> Result<Self> {
        if let Ok(contents) = std::fs::read_to_string(file_path) {
            let mut registry: Registry = serde_json::from_str(&contents)?;
            registry.file_path = file_path.to_string(); // restore file_path
            Ok(registry)
        } else {
            Ok(Registry::new(0, file_path))
        }
    }
}
