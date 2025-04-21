import asyncio
import sys

from filplus_autocap.blockchain_utils.transaction import Tx, TxProcessor
from filplus_autocap.contracts.bots.revenue_bot import RevenueBot
from filplus_autocap.contracts.bots.datacap_bot import DatacapBot
from filplus_autocap.contracts.bots.bot import Bot
from filplus_autocap.utils.logger import get_logger
from filplus_autocap.blockchain_utils.currencies import FIL, DAT

class MasterBot(Bot):
    """
    The MasterBot orchestrates the auction rounds by coordinating with the RevenueBot and DatacapBot.
    It handles the distribution of FIL and Datacap to the Storage Providers (SPs) based on their contributions
    and processes protocol and burn fees.
    """

    def __init__(
        self,
        address: str,
        revenue_bot: RevenueBot,
        datacap_bot: DatacapBot,
        master_fee_ratio: FIL = FIL(0.1),
        protocol_fee_ratio: FIL = FIL(0.1),
        datacap_distribution_round: DAT = DAT(1000.0),
        auction_duration: float = 10.0,
        protocol_wallet_address: str = "f1_protocol_wallet",
        burn_address: str = "f099",
        processor: TxProcessor = None
    ):
        """
        Initializes the MasterBot with the provided parameters for managing auction rounds and fee distributions.
        
        Args:
            address (str): The address of the MasterBot.
            revenue_bot (RevenueBot): An instance of the RevenueBot to manage FIL distribution.
            datacap_bot (DatacapBot): An instance of the DatacapBot to manage Datacap distribution.
            master_fee_ratio (FIL): The ratio of the total FIL to be taken as the MasterBot's fee.
            protocol_fee_ratio (FIL): The ratio of the total FIL to be taken as the protocol fee.
            datacap_distribution_round (DAT): The amount of Datacap to be distributed per auction round.
            auction_duration (float): The duration of each auction round in epochs.
            protocol_wallet_address (str): The address to send protocol fees to.
            burn_address (str): The address where the burned FIL will be sent.
            processor (TxProcessor): A processor that handles transaction execution.
        """
        self.address = address
        self.revenue_bot = revenue_bot
        self.datacap_bot = datacap_bot
        self.master_fee_ratio = FIL(master_fee_ratio)
        self.protocol_fee_ratio = FIL(protocol_fee_ratio)
        self.datacap_distribution_round = DAT(datacap_distribution_round)
        self.auction_duration = auction_duration
        self.protocol_wallet_address = protocol_wallet_address
        self.burn_address = burn_address
        self.processor = processor
        self.header = "[🤖 MasterBot]"
        self.logger = get_logger("MasterBotLogger")

    def execute_auction_round(self) -> None:
        """
        Executes a single auction round, distributing FIL and Datacap to verified SPs,
        while handling burn and protocol fees.
        """
        auction_data = self.revenue_bot.drain_auction()
        total_fil = sum(auction_data.values())
    
        if total_fil == FIL(0):
            return []

        reward_txs = []
        reward_txs += self.calculate_sp_rewards(auction_data, total_fil)
        reward_txs += self.generate_protocol_and_burn_txs(total_fil, auction_data)
        reward_txs += self.handle_unverified_sp_redirection()

        self.log_and_dispatch_transactions(reward_txs)

        return

    def calculate_sp_rewards(self, auction_data: dict, total_fil: float) -> list:
        """Generates refund and datacap reward transactions for each SP."""
        txs = []
        refund_total = FIL(0)
    
        for sp_address, contribution in auction_data.items():
            c_i = contribution / total_fil
            refund_amount = (1 - self.master_fee_ratio) * contribution
            datacap_amount = c_i * self.datacap_distribution_round
            refund_total += refund_amount
    
            # Instruct revenuebot to craft the tx
            tx = self.revenue_bot.create_fil_tx(recipient_address=sp_address, fil_amount=FIL(refund_amount), message= "Refund after auction")
            txs.append(self.sign_tx(tx))
       
            # Instruct databot to craft the tx
            tx = self.datacap_bot.create_datacap_tx(recipient_address=sp_address, datacap_amount=DAT(datacap_amount), message = f"Datacap issued: {datacap_amount:.2f}")
            txs.append(self.sign_tx(tx))
    
        self.refund_total = refund_total  # Store for later use
        return txs
    
    
    def generate_protocol_and_burn_txs(self, total_fil: float, auction_data: dict) -> list:
        """Generates burn and protocol fee transactions."""
        refund_total = getattr(self, "refund_total", FIL(0))
        leftover = total_fil - refund_total
        burn = leftover * (1 - self.protocol_fee_ratio)
        fee = leftover * self.protocol_fee_ratio
        
        #Instruct the Revenue bot to craft the txs
        burn_tx = self.revenue_bot.create_fil_tx(recipient_address=self.burn_address, fil_amount=FIL(burn), message="Burned FIL")
        protocol_tx = self.revenue_bot.create_fil_tx(recipient_address=self.protocol_wallet_address, fil_amount=FIL(fee), message="Protocol fee")
    
        return [self.sign_tx(burn_tx), self.sign_tx(protocol_tx)]
    
    
    def handle_unverified_sp_redirection(self) -> list:
        """Returns any redirection txs created by RevenueBot from unverified SPs."""
        signed_txs = []
        for tx in list(self.revenue_bot.outgoing_txs):
            signed_txs.append(self.sign_tx(tx))

        return signed_txs
    
    
    def log_and_dispatch_transactions(self, txs: list) -> None:
        """Logs and sends all generated transactions."""
        for tx in txs:
            self.logger.info(f"{self.header}   Tx: {tx}")
            self.processor.send([tx])

    async def run_auction(self, time_vector: list[float]):
        """
        Runs the auction simulation asynchronously, executing auction rounds based on a time vector.

        Args:
            time_vector (list[float]): A list of time steps (epochs) to simulate the auction over time.
        """
        self.logger.info(self.header + f" ⏳ Starting auction simulation. Auction duration: {self.auction_duration} epochs")
        self.print_initial_state()
        round_number = 0

        for t in time_vector:
            self.logger.info(f"[time: {t} epochs] ⏱️ Tick...")

            # Check if it's time for a new auction round
            if t % self.auction_duration == 0 and t != 0:
                if self.datacap_bot.get_datacap_balance() < self.datacap_distribution_round:
                    self.logger.warning(f"[time: {t} epochs] ⚠️ Not enough datacap to run auction round.")
                    break  # Stop the auction if there's not enough datacap

                self.log_blank_line()
                self.logger.info(f"{self.header} 🚀 Executing auction round number {round_number}")
                self.print_initial_state()
                self.execute_auction_round()  # Execute the auction round
                round_number += 1
                self.print_final_state()

            await asyncio.sleep(1)

        self.logger.info(f"{self.header} ⏳ Auction simulation completed.")
        self.print_final_state()

    async def run_auction_in_background(self, time_vector):
        """Runs the auction simulation in the background."""
        try:
            await self.run_auction(time_vector)
        except asyncio.CancelledError:
            self.logger.info("[🛑] Auction loop was cancelled.")
            return  # Cleanly handle the cancellation

    def print_initial_state(self):
        """
        Logs the initial state of the system before the auction begins, including wallet balances 
        and the RevenueBot's auction state.
        """
        self.logger.info(self.header + " 🔛 Initial System State")
        self.logger.info(self.header + " " + "=" * 80)
        self.logger.info(self.header + " 📦 Wallet Balances at the start:")
        for addr, wallet in self.processor.wallets.items():
            self.logger.info(f"{self.header}     - {wallet}")
        self.logger.info(self.header + " 📊 RevenueBot Auction State at the start:")
        if self.revenue_bot.current_auction:
            for sp, amount in self.revenue_bot.current_auction.items():
                self.logger.info(f"{self.header}     - SP {sp} → {amount:.2f} FIL")
        else:
            self.logger.info(f"{self.header}     - ✅ No active contributors. Auction cleared.")
        self.logger.info(self.header + " " + "=" * 80 + '\n')

    def print_final_state(self):
        """
        Logs the final state of the system after the auction has completed.
        """
        self.logger.info(self.header + " 🔚 Final System State")
        self.logger.info(self.header + " " + "=" * 80)
        self.logger.info(self.header + " 📦 Wallet Balances:")
        for addr, wallet in self.processor.wallets.items():
            self.logger.info(f"{self.header}     - {wallet}")
        self.logger.info(self.header + " 📊 RevenueBot Auction State:")
        if self.revenue_bot.current_auction:
            for sp, amount in self.revenue_bot.current_auction.items():
                self.logger.info(f"{self.header}     - SP {sp} → {amount:.2f} FIL")
        else:
            self.logger.info(f"{self.header}     - ✅ No active contributors. Auction cleared.")
        self.logger.info(self.header + " " + "=" * 80 + '\n')

    def log_blank_line(self):
        """Logs a blank line to help with formatting in logs."""
        self.logger.handlers[0].stream.write("\n")
        self.logger.handlers[0].flush()
