use std::thread::{sleep};
use std::time::Duration;
use reqwest::blocking::Client;
use anyhow::Result;
use cid::Cid;
use std::str::FromStr;

use crate::wallet::Wallet;
use crate::rpc::{get_chain_head_block_number, get_block_info, send_fil_to, Connection, create_datacap_allocation};
use crate::transaction::filter_incoming_txs;
use crate::auction::{Auction, AuctionReward};
use crate::registry::{Registry};
use crate::utils::{format_datacap_size, fil_to_atto_string};
use crate::allocation::{AllocationRequest, AllocationRequests, Size, craft_allocation_request, build_transfer_from_payload};
use crate::constants::{BOT_BURN_FEE, BOT_DATACAP_ISSUANCE_ROUND, BOT_AUCTION_INTERVAL};

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
        let auction_interval = BOT_AUCTION_INTERVAL;
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

    pub fn run(&mut self) {
        println!("\n🤖 MasterBot started at block {}", self.last_block);

        let mut blocks_left = self.auction_interval;
        loop {
            let current_block = get_chain_head_block_number(&self.connection).unwrap_or(self.last_block);

            if current_block > self.last_block {
                if let Ok(block) = get_block_info(&self.connection, &current_block) {
                    println!("📦 MasterBot processing block {}", current_block);
                    self.auction.block_number = current_block;
                    self.process_block(block, current_block);
                    self.last_block = current_block;

                    blocks_left -= 1;
                    println!("⌛ Waiting for next auction round in {} blocks", blocks_left);
                }
            }

            if blocks_left == 0 {
                self.last_auction_block = current_block;
                self.execute_auction_round();
                blocks_left = self.auction_interval;
            }

            sleep(Duration::from_secs(5));
        }
    }

    fn process_block(&mut self, block: serde_json::Value, current_block_number: u64) {
        
        let transactions = filter_incoming_txs(&block, &self.wallet.address, current_block_number);
        for tx in &transactions {
            println!("🪙  Detected tx: {} from {} with {} FIL", tx.cid, tx.from, tx.value_fil);
            self.auction.transactions.push(tx.clone());
        }

        self.auction.save(); // Save to disk or memory
    }

    fn execute_auction_round(&mut self) -> Result<()> {
        println!("🚀 Executing auction round...");

        if self.auction.transactions.is_empty() {
            println!("✅ No transactions. Skipping auction.");
        } else {
            // Compute auction datacap rewards
            let (total_fil_auction, rewards) = match self.compute_rewards() {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("❌ Failed to compute rewards: {}", e);
                    return Err(e); // or return Ok(()) if you want to continue silently
                }
            };
            // Update credit registry
            self.update_registry(rewards);
            // Use the available datacap to perform the allocations
            self.create_allocations();
            self.burn_fees(total_fil_auction);
        }

        self.auction.reset();
        println!("✅ Auction cleared.");
        Ok(())
    }
    
    //
    fn compute_rewards(&self) -> Result<(f64, Vec<AuctionReward>)> {
        let txs = &self.auction.transactions;
    
        // Total FIL contributed in this auction round by all SPs
        let total_fil: f64 = txs.iter().map(|tx| tx.value_fil).sum();
    
        if total_fil == 0.0 {
            println!("✅ Total contribution is zero. Skipping round.");
            return Ok((0.0, vec![]));
        }
    
        let mut rewards: Vec<AuctionReward> = Vec::new();
        let mut rewarded_total = 0u64;
    
        // First pass: floor each allocation
        for tx in txs {
            let weight = tx.value_fil / total_fil;
            let reward = (weight * BOT_DATACAP_ISSUANCE_ROUND as f64).floor() as u64;
            rewarded_total += reward;
            let reward_value = AuctionReward::new(tx.from.clone(), reward);
            rewards.push(reward_value);
        }
    
        // Distribute leftover bytes (greedy round-robin)
        let mut remaining = BOT_DATACAP_ISSUANCE_ROUND - rewarded_total;
        for auction_reward in rewards.iter_mut() {
            if remaining == 0 {
                break;
            }
            auction_reward.reward += 1;
            remaining -= 1;
        }
        
        for auction_reward in rewards.iter(){
            println!("💸 {} gained {} DataCap", auction_reward.address, format_datacap_size(auction_reward.reward));
        }

        Ok((total_fil, rewards))
    }

    fn update_registry(&mut self, rewards: Vec<AuctionReward>) -> Result<()> {
        self.registry.block_number = self.last_auction_block;
    
        for reward in rewards {
            *self
                .registry
                .credits
                .entry(reward.address)
                .or_insert(0) += reward.reward;
        }
    
        self.registry.save()?;
        Ok(())
    }


    fn create_allocations(&self) -> Result<()> {
    
        // Run through the txs, check the credit of the SP
        for tx in self.auction.transactions.iter() {
            if let Some(sp_credit) = self.registry.credits.get(&tx.from) {
                println!("📦 SP {} has {} bytes of credit", tx.from, sp_credit);
                // TODO: use sp_credit to decide on allocation
            } else {
                println!("⚠️ SP {} has no credit entry", tx.from);
            }
        }
        // If sufficient for the metadata of the tx, create the allocation, update the credit
        // If not sufficient skip it
        // Note: pay attention that an SP could send you metadata of the same deal. Beware of this when you create allocations
        Ok(())
    }

    fn burn_fees(&self, total_fil: f64) -> Result<()> {
        // Compute total burn fee and send it
        let burn_fee = BOT_BURN_FEE * total_fil;
        let burn_fee_atto = fil_to_atto_string(burn_fee);
        match send_fil_to(&self.connection, &self.wallet, "f099", &burn_fee_atto) {
            Ok(cid) => println!("🔥 Sent {} aFIL to burn address. CID: {}", burn_fee_atto, cid),
            Err(e) => eprintln!("❌ Failed to send burn fee: {}", e),
        }
    
        Ok(())
    }


}
