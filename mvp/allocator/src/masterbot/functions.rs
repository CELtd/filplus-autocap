use std::thread::{sleep};
use std::time::Duration;

use anyhow::{Result, Context};
use std::collections::HashSet;
use log::{info, warn, error};


use crate::rpc::{get_chain_head_block_number, get_block_info, send_fil_to,  create_datacap_allocation, fetch_datacap_balance, request_datacap};
use crate::transaction::filter_incoming_txs;
use crate::auction::{AuctionReward};
use crate::utils::{format_datacap_size, fil_to_atto_string};
use crate::allocation::{craft_transfer_from_payload};
use crate::constants::bot::{BURN_FEE, DATACAP_ISSUANCE_ROUND};
use crate::masterbot::MasterBot;


impl MasterBot {
    /// Start the main loop of the MasterBot.
    /// Polls blocks, processes incoming txs, and runs auction rounds every `auction_interval`.
    pub fn run(&mut self) -> Result<()> {
        info!("🤖 MasterBot started at block {}", self.last_block);

        let mut blocks_left = self.auction_interval;
        let mut first_block = true;

        loop {
            let current_block = get_chain_head_block_number(&self.connection).unwrap_or(self.last_block);

            if current_block > self.last_block || first_block {
                if let Ok(block) = get_block_info(&self.connection, &current_block) {
                    info!("📦 MasterBot processing block {}", current_block);
                    self.auction.block_number = current_block;
                    self.process_block(block, current_block);
                    self.last_block = current_block;

                    blocks_left -= 1;
                    info!("⌛ Waiting for next auction round in {} blocks", blocks_left);
                    //// Fetch datacap from the allocator if we had some allocations
                    //if blocks_left == self.auction_interval - 2  {
                    //    let current_datacap_balance = fetch_datacap_balance(&self.connection, &self.wallet.address).unwrap_or(0);
                    //}
                    first_block = false;
                }
            }

            if blocks_left == 0 {
                self.last_auction_block = current_block;
                // Run an auction round to distribute datacap and burn fees
                let datacap_allocated = self.execute_auction_round().unwrap_or(0);
                if datacap_allocated > 0 {
                    info!("ℹ️  Masterbot spent {} of DataCap. Requesting {} of DataCap to Allocator", datacap_allocated, datacap_allocated);
                    let tx_hash = request_datacap(
                        &self.connection,
                        &self.metallocator_contract_address,
                        &self.allocator_private_key,
                        &hex::decode("01c3c98d44ea0cc6094819ee0735c51b92e8fc9e38")?,
                        datacap_allocated,
                    )?;

                    info!("🛰️  DataCap requested. Tx id Eth {}", tx_hash);
                }
                blocks_left = self.auction_interval;
            }

            // Fetch datacap from the allocator if we had some allocations

            //sleep(Duration::from_secs(1)); //removed in testnet
        }
    }

    /// Processes a new block: filters incoming txs and stores them for the current auction.
    fn process_block(&mut self, block: serde_json::Value, current_block_number: u64) {
        
        let transactions = filter_incoming_txs(&block, &self.wallet.address, current_block_number, &self.connection);
        for tx in &transactions {
            info!("🪙  Detected tx: {} from {} with {} FIL", tx.cid, tx.from, tx.value_fil);
            self.auction.transactions.push(tx.clone());
        }

        self.auction.save(); // Update auction state
    }

    /// Runs the auction: rewards SPs, allocates datacap, and burns FIL.
    fn execute_auction_round(&mut self) -> Result<u64> {

        info!("🚀 Executing auction round...");
        let mut datacap_allocated: u64 = 0;

        if self.auction.transactions.is_empty() {
            info!("ℹ️  No transactions. Skipping auction.");
        } else {
            // Compute auction datacap rewards
            let (total_fil_auction, rewards) = match self.compute_rewards() {
                Ok(result) => result,
                Err(e) => {
                    error!("❌ Failed to compute rewards: {}", e);
                    return Err(e); // or return Ok(()) if you want to continue silently
                }
            };
            // Update credit registry
            self.update_registry(rewards);
            // Use the available datacap to perform the allocations
            datacap_allocated = self.create_allocations()?;
            // Send the fee to the burn address
            self.burn_fees(total_fil_auction);
        }

        // Reset the auction
        self.auction.reset();
        info!("✅ Auction cleared.");

        Ok(datacap_allocated)
    }
    
    /// Computes datacap rewards based on FIL contributions.
    fn compute_rewards(&self) -> Result<(f64, Vec<AuctionReward>)> {
        let txs = &self.auction.transactions;
    
        // Total FIL contributed in this auction round by all SPs
        let total_fil: f64 = txs.iter().map(|tx| tx.value_fil).sum();
    
        if total_fil == 0.0 {
            info!("ℹ️ Total contribution is zero. Skipping round.");
            return Ok((0.0, vec![]));
        }
    
        let mut rewards: Vec<AuctionReward> = Vec::new();
        let mut rewarded_total = 0u64;
    
        // Proportional allocation based on FIL contribution
        // First: floor each allocation
        for tx in txs {
            let reward: u64;
            if tx.value_fil == 0.0 { 
                reward = 0 as u64;
            }
            else{
                let weight = tx.value_fil / total_fil;
                reward = (weight * DATACAP_ISSUANCE_ROUND as f64).floor() as u64;
                rewarded_total += reward;
            }
            let reward_value = AuctionReward::new(tx.from.clone(), reward);
            rewards.push(reward_value);
        }
    
        // Distribute leftover bytes (greedy round-robin)
        let mut remaining = DATACAP_ISSUANCE_ROUND - rewarded_total;
        for auction_reward in rewards.iter_mut() {
            if remaining == 0 {
                break;
            }
            if auction_reward.reward == 0 {
                continue; // skip SPs with 0 FIL contribution
            }
            auction_reward.reward += 1;
            remaining -= 1;
        }
        
        for auction_reward in rewards.iter(){
            info!("💸 {} gained {} DataCap", auction_reward.address, format_datacap_size(&auction_reward.reward));
        }

        Ok((total_fil, rewards))
    }

     /// Updates the credit registry with datacap rewards earned by SPs.
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

    /// Allocates verified datacap to SPs based on their deal metadata and credit.
    fn create_allocations(&mut self) -> Result<u64> {

        let mut datacap_allocated: u64 = 0;
        let mut seen_cids: HashSet<String> = HashSet::new();


        for tx in self.auction.transactions.iter() {
            let sender = &tx.from;

            if let Some(metadata) = &tx.metadata {
                // Avoid processing the same CID twice
                let cid_str = metadata.data.to_string();
                if seen_cids.contains(&cid_str) {
                    warn!("⚠️  Skipping duplicate deal CID: {}", cid_str);
                    continue;
                }

                // Check if SP has credit
                if let Some(sp_credit) = self.registry.credits.get(sender) {
                    info!("💰 SP {} has {} bytes of credit", sender, sp_credit);

                    let datacap_required = metadata.size.0;
                    if *sp_credit >= datacap_required {
                        // Craft parameters to be ingested by tranfer function of DataCap Actor
                        let transfer_params_bytes = craft_transfer_from_payload(
                            &metadata.provider.to_string(),
                            &metadata.data.to_string(),
                            &datacap_required,
                            &metadata.term_min,
                            &metadata.term_max,
                            &self.last_auction_block,
                            &metadata.size.0.to_string(),
                        )?;

                        // Send allocation tx
                        let cid = create_datacap_allocation(transfer_params_bytes, &self.connection, &self.wallet)?;
                        info!("✅ Allocation created for SP {} → Tx CID: {:?}", sender, cid);

                        // Deduct credit
                        self.registry.credits.insert(sender.clone(), sp_credit - metadata.size.0);
                       
                        // Re-fetch updated credit for printing
                        if let Some(updated_credit) = self.registry.credits.get(sender) {
                            info!("💰 SP {} has {} bytes of credit remaining", sender, updated_credit);
                        }

                        // Track used CID
                        seen_cids.insert(cid_str);

                        // update allocated datacap
                        datacap_allocated += metadata.size.0;
                    } else {
                        warn!(
                            "⚠️  SP {} has insufficient credit for allocation. Required: {}, Available: {}",
                            sender, metadata.size.0, sp_credit
                        );
                    }
                } else {
                    warn!("⚠️  SP {} has no credit entry", sender);
                }
            } else {
                warn!("⚠️  Transaction from {} has no metadata. Skipping.", sender);
            }
        }

        // Update registry
        self.registry.save()?;

        Ok(datacap_allocated)
    }

    /// Burns a portion of the contributed FIL as a protocol fee.
    fn burn_fees(&self, total_fil: f64) -> Result<()> {
        // Compute total burn fee and send it
        let burn_fee = BURN_FEE * total_fil;
        let burn_fee_atto = fil_to_atto_string(burn_fee);
        match send_fil_to(&self.connection, &self.wallet, "f099", &burn_fee_atto) {
            Ok(cid) => info!("🔥 Sent {} aFIL to burn address. CID: {}", burn_fee_atto, cid),
            Err(e) => error!("❌ Failed to send burn fee: {}", e),
        }
    
        Ok(())
    }


}

