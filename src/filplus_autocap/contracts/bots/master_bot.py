from filplus_autocap.blockchain_utils.transaction import Tx, TxProcessor
from filplus_autocap.contracts.bots.revenue_bot import RevenueBot
from filplus_autocap.contracts.bots.datacap_bot import DatacapBot
import asyncio

class MasterBot:
    def __init__(
        self,
        address: str,
        revenue_bot: RevenueBot,
        datacap_bot: DatacapBot,
        master_fee_ratio: float = 0.1,
        protocol_fee_ratio: float = 0.1,
        datacap_distribution_round: float = 1000.0,
        auction_duration: float = 10.0,  # duration between auctions (in seconds or blocks)
        protocol_wallet_address: str = "f1_protocol_wallet",
        burn_address: str = "f099",
        processor: TxProcessor = None
    ):
        self.address = address
        self.revenue_bot = revenue_bot
        self.datacap_bot = datacap_bot
        self.master_fee_ratio = master_fee_ratio
        self.protocol_fee_ratio = protocol_fee_ratio
        self.datacap_distribution_round = datacap_distribution_round
        self.auction_duration = auction_duration
        self.protocol_wallet_address = protocol_wallet_address
        self.burn_address = burn_address
        self.processor = processor
        self.header = "[🤖 MasterBot]"

    def execute_auction_round(self) -> list[Tx]:
        auction_data = self.revenue_bot.drain_auction()
        total_fil = sum(auction_data.values())
        reward_txs = []

        if total_fil == 0:
            return []

        for sp_address, contribution in auction_data.items():
            c_i = contribution / total_fil
            refund_amount = (1 - self.master_fee_ratio) * contribution
            datacap_amount = c_i * self.datacap_distribution_round
            total_fil -= refund_amount

            reward_txs.append(
                Tx(
                    sender=self.revenue_bot.address,
                    recipient=sp_address,
                    fil_amount=refund_amount,
                    datacap_amount=0.0,
                    signers=[self.revenue_bot.address, self.address],
                    message="Refund after auction",
                )
            )

            reward_txs.append(
                Tx(
                    sender=self.datacap_bot.datacap_wallet.address,
                    recipient=sp_address,
                    datacap_amount=datacap_amount,
                    fil_amount=0.0,
                    signers=[self.datacap_bot.address, self.address],
                    message=f"Datacap issued: {datacap_amount:.2f}",
                )
            )

        # Fees and burn
        leftover_balance = total_fil
        burn_amount = leftover_balance * (1 - self.protocol_fee_ratio)
        protocol_fee_amount = leftover_balance * self.protocol_fee_ratio

        reward_txs.append(
            Tx(
                sender=self.revenue_bot.address,
                recipient=self.burn_address,
                fil_amount=burn_amount,
                datacap_amount=0.0,
                signers=[self.revenue_bot.address, self.address],
                message="Burned FIL",
            )
        )

        reward_txs.append(
            Tx(
                sender=self.revenue_bot.address,
                recipient=self.protocol_wallet_address,
                fil_amount=protocol_fee_amount,
                datacap_amount=0.0,
                signers=[self.revenue_bot.address, self.address],
                message="Protocol fee",
            )
        )

        return reward_txs

    async def run_auction(self, time_vector: list[float]):
        """
        Simulates auction rounds over a given time vector.
        Executes an auction every self.auction_duration units.
        """
        # Print initial state as soon as auction starts
        print(self.header + " ⏳ Starting auction simulation. Duration:", self.auction_duration)
        self.print_initial_state()
        round_number = 0
    
        for t in time_vector:
            print(f"[time: {t} epochs] ⏱️ Tick...")
    
            # Perform auction only at correct intervals
            if t % self.auction_duration == 0 and t != 0:
                if self.datacap_bot.get_datacap_balance() < self.datacap_distribution_round:
                    print(f"[time: {t} epochs] ⚠️ Not enough datacap to run auction round.")
                    break
    
                print(f"\n[🤖 MasterBot] 🚀 Executing auction round number {round_number}")
                txs = self.execute_auction_round()
                for tx in txs:
                    print(f"[🤖 MasterBot]   Tx: {tx}")  # Indented Tx
                    self.processor.send([tx])
                round_number += 1
                self.print_final_state()
    
            await asyncio.sleep(1)  # Simulated delay between time steps
        print("[🤖 MasterBot] ⏳ Auction simulation completed.")
        self.print_final_state()

    def print_initial_state(self):
        # Print initial state when the auction starts
        print("\n" + self.header + " 🔛 Initial System State")
        print(self.header + " " + "=" * 80)
        print(self.header + " 📦 Wallet Balances at the start:")
        for addr, wallet in self.processor.wallets.items():
            print(f"{self.header}     - {wallet}")  # Added space here for indentation
        print(self.header + " 📊 RevenueBot Auction State at the start:")
        if self.revenue_bot.current_auction:
            for sp, amount in self.revenue_bot.current_auction.items():
                print(f"{self.header}     - SP {sp} → {amount:.2f} FIL")  # Added space here for indentation
        else:
            print(f"{self.header}     - ✅ No active contributors. Auction cleared.")  # Added space here for indentation
        print(self.header + " " + "=" * 80 + '\n')

    def print_final_state(self):
        # Print the final state when auction is complete
        print("\n" + self.header + " 🔚 Final System State")
        print(self.header + " " + "=" * 80)
        print(self.header + " 📦 Wallet Balances:")
        for addr, wallet in self.processor.wallets.items():
            print(f"{self.header}     - {wallet}")  # Added space here for indentation
        print(self.header + " 📊 RevenueBot Auction State:")
        if self.revenue_bot.current_auction:
            for sp, amount in self.revenue_bot.current_auction.items():
                print(f"{self.header}     - SP {sp} → {amount:.2f} FIL")  # Added space here for indentation
        else:
            print(f"{self.header}     - ✅ No active contributors. Auction cleared.")  # Added space here for indentation
        print(self.header + " " + "=" * 80 + '\n')
