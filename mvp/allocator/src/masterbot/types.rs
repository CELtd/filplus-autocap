use anyhow::Result;

use crate::wallet::Wallet;
use crate::rpc::Connection;
use crate::auction::Auction;
use crate::registry::Registry;
use crate::constants::bot::AUCTION_INTERVAL;

/// MasterBot coordinates auction rounds, tracks state, and performs DataCap allocations.
pub struct MasterBot {
    /// The wallet used to receive FIL and issue allocations.
    pub wallet: Wallet,

    /// Connection to the Filecoin RPC endpoint.
    pub connection: Connection,

    /// The most recent block number processed.
    pub last_block: u64,

    /// The block number when the last auction round was executed.
    pub last_auction_block: u64,

    /// The auction instance, which accumulates and processes SP contributions.
    pub auction: Auction,

    /// The number of blocks between each auction round.
    pub auction_interval: u64,

    /// Registry of SP credit balances and the current block state.
    pub registry: Registry,
}

impl MasterBot {
    /// Constructs a new MasterBot instance.
    ///
    /// Loads auction and registry state from file or initializes new ones.
    ///
    /// # Arguments
    /// * `wallet` - The wallet managing funds and signing transactions.
    /// * `connection` - RPC connection to the Filecoin node.
    /// * `start_block` - Current head block when starting the bot.
    /// * `auction_file` - Path to file storing auction state.
    /// * `registry_file` - Path to file storing registry state.
    pub fn new(
        wallet: Wallet,
        connection: Connection,
        start_block: u64,
        auction_file: &str,
        registry_file: &str,
    ) -> Result<Self> {
        // Restore persisted state or create new auction and registry
        let auction = Auction::load_or_new(auction_file)?;
        let registry = Registry::load_or_new(registry_file)?;

        Ok(Self {
            wallet,
            connection,
            last_block: start_block,
            last_auction_block: start_block,
            auction,
            auction_interval: AUCTION_INTERVAL,
            registry,
        })
    }
}
