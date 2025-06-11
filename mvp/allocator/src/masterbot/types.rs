use anyhow::Result;

use crate::auction::Auction;
use crate::registry::Registry;
use crate::constants::bot::AUCTION_INTERVAL;
use crate::runtime::config::AppConfig;
use crate::wallet::{Wallet, load_or_create_wallet};
use crate::rpc::{Connection, fetch_balance, fetch_datacap_balance, get_chain_head_block_number};
use crate::utils::format_datacap_size;

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

    /// Allocator address
    allocator_address_hex: String,

    /// Allocator private key
    allocator_private_key: String,

    /// Contract Metallocator Address
    metallocator_contract_address: String,
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
    pub fn new(config:AppConfig) -> Result<Self> {

        // Establish RPC connection
        let connection = Connection::new(&config.rpc_url);

        // Load or create the wallet
        let wallet = load_or_create_wallet(&config.wallet_file)?;

        // Print wallet address in testnet format
        let testnet_address = wallet.address.replacen("f1", "t1", 1);
        log::info!("📬 Filecoin wallet (testnet): {}", testnet_address);

        // Fetch and log wallet FIL balance
        let balance = fetch_balance(&connection, &wallet.address)?;
        log::info!("💰 FIL balance: {} attoFIL", balance);

        // Fetch and log wallet DataCap balance
        let datacap = fetch_datacap_balance(&connection, &wallet.address).unwrap_or(0);
        log::info!("✅ Datacap balance: {}", format_datacap_size(&datacap));

        // Fetch latest chain head block number
        let current_block = get_chain_head_block_number(&connection).unwrap_or(0);

        // Restore persisted state or create new auction and registry
        let auction = Auction::load_or_new(&config.auction_file)?;
        let registry = Registry::load_or_new(&config.registry_file)?;
        
        // Load allocator and metallocator contract variables
        let allocator_address_hex = config.allocator_address_hex;
        let allocator_private_key = config.allocator_private_key;
        let metallocator_contract_address = config.metallocator_contract_address; 
        

        Ok(Self {
            wallet,
            connection,
            last_block: current_block,
            last_auction_block: current_block,
            auction,
            auction_interval: AUCTION_INTERVAL,
            registry,
            allocator_address_hex,
            allocator_private_key,
            metallocator_contract_address,
        })
    }
}
