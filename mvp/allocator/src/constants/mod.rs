// Filecoin constants
pub mod filecoin {
    pub const EPOCHS_PER_DAY: i64 = 2880;
}

// Datacap Actor
pub mod datacap_actor{
    pub const DATACAP_ACTOR_ID: u64 = 7; 
    pub const DATACAP_TRANSFER_FUNCTION_ID: u64 = 80475954; 
}

// Bot constants
pub mod bot {
    pub const BURN_FEE: f64 = 0.50;
    //pub const DATACAP_ISSUANCE_ROUND: u64 = 1024;
    pub const DATACAP_ISSUANCE_ROUND: u64 = 1280;
    pub const AUCTION_INTERVAL: u64 = 15;
}

// Gas cost of operations
pub mod gas {
    pub const SEND_FIL_GAS: u64 = 750_000;//730_463;
    pub const ALLOCATION_GAS: u64 = 20_000_000;//17_291_879;
}

