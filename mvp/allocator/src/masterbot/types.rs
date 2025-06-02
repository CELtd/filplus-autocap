use anyhow::Result;

use crate::wallet::Wallet;
use crate::rpc::Connection;
use crate::auction::{Auction};
use crate::registry::{Registry};
use crate::constants::bot::{AUCTION_INTERVAL};

pub struct MasterBot {
    pub wallet: Wallet,
    pub connection: Connection,
    pub last_block: u64,
    pub last_auction_block: u64,
    pub auction: Auction,
    pub auction_interval: u64,
    pub registry: Registry,
}

impl MasterBot {
    pub fn new(wallet: Wallet, connection: Connection, start_block: u64, auction_file: &str, registry_file: &str) -> Result<Self,anyhow::Error> {
        // Load auction and registry
        let auction = Auction::load_or_new(auction_file)?;
        let registry = Registry::load_or_new(registry_file)?;
        let auction_interval = AUCTION_INTERVAL;
        Ok(Self {
            wallet,
            connection,
            last_block: start_block,
            last_auction_block: start_block,
            auction,
            auction_interval,
            registry,
        })
    }
}