import asyncio
import sys

from filplus_autocap.blockchain_utils.transaction import Tx, TxProcessor
from filplus_autocap.contracts.bots.revenue_bot import RevenueBot
from filplus_autocap.contracts.bots.datacap_bot import DatacapBot
from filplus_autocap.utils.logger import get_logger
from filplus_autocap.blockchain_utils.currencies import FIL, DAT


logger = get_logger("MasterBotLogger")

class MasterBot:
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

    def execute_auction_round(self) -> list[Tx]:
        auction_data = self.revenue_bot.drain_auction()
        total_fil = sum(auction_data.values())
        reward_txs = []

        if total_fil == FIL(0):
            return 

        refund = FIL(0)
        for sp_address, contribution in auction_data.items():
            c_i = contribution / total_fil
            refund_amount = (1 - self.master_fee_ratio) * contribution
            datacap_amount = c_i * self.datacap_distribution_round
            refund += refund_amount

            reward_txs.append(
                Tx(
                    sender=self.revenue_bot.address,
                    recipient=sp_address,
                    fil_amount=FIL(refund_amount),
                    datacap_amount=DAT(0.0),
                    signers=[self.revenue_bot.address, self.address],
                    message="Refund after auction",
                )
            )

            reward_txs.append(
                Tx(
                    sender=self.datacap_bot.datacap_wallet.address,
                    recipient=sp_address,
                    datacap_amount=DAT(datacap_amount),
                    fil_amount=FIL(0.0),
                    signers=[self.datacap_bot.address, self.address],
                    message=f"Datacap issued: {datacap_amount:.2f}",
                )
            )

        leftover_balance = total_fil - refund
        burn_amount = leftover_balance * (1 - self.protocol_fee_ratio)
        protocol_fee_amount = leftover_balance * self.protocol_fee_ratio

        reward_txs.append(
            Tx(
                sender=self.revenue_bot.address,
                recipient=self.burn_address,
                fil_amount=FIL(burn_amount),
                datacap_amount=DAT(0.0),
                signers=[self.revenue_bot.address, self.address],
                message="Burned FIL",
            )
        )

        reward_txs.append(
            Tx(
                sender=self.revenue_bot.address,
                recipient=self.protocol_wallet_address,
                fil_amount=FIL(protocol_fee_amount),
                datacap_amount=DAT(0.0),
                signers=[self.revenue_bot.address, self.address],
                message="Protocol fee",
            )
        )
        # Execute txs
        for tx in reward_txs:
            logger.info(f"{self.header}   Tx: {tx}")
            self.processor.send([tx])

        return

    async def run_auction(self, time_vector: list[float]):
        logger.info(self.header + f" ⏳ Starting auction simulation. Duration: {self.auction_duration}")
        self.print_initial_state()
        round_number = 0

        for t in time_vector:
            logger.info(f"[time: {t} epochs] ⏱️ Tick...")

            if t % self.auction_duration == 0 and t != 0:
                if self.datacap_bot.get_datacap_balance() < self.datacap_distribution_round:
                    logger.warning(f"[time: {t} epochs] ⚠️ Not enough datacap to run auction round.")
                    break

                logger.info(f"{self.header} 🚀 Executing auction round number {round_number}")
                self.print_initial_state()
                self.execute_auction_round()
                round_number += 1
                self.print_final_state()

            await asyncio.sleep(1)

        logger.info(f"{self.header} ⏳ Auction simulation completed.")
        self.print_final_state()

    def print_initial_state(self):
        logger.info(self.header + " 🔛 Initial System State")
        logger.info(self.header + " " + "=" * 80)
        logger.info(self.header + " 📦 Wallet Balances at the start:")
        for addr, wallet in self.processor.wallets.items():
            logger.info(f"{self.header}     - {wallet}")
        logger.info(self.header + " 📊 RevenueBot Auction State at the start:")
        if self.revenue_bot.current_auction:
            for sp, amount in self.revenue_bot.current_auction.items():
                logger.info(f"{self.header}     - SP {sp} → {amount:.2f} FIL")
        else:
            logger.info(f"{self.header}     - ✅ No active contributors. Auction cleared.")
        logger.info(self.header + " " + "=" * 80 + '\n')

    def print_final_state(self):
        logger.info(self.header + " 🔚 Final System State")
        logger.info(self.header + " " + "=" * 80)
        logger.info(self.header + " 📦 Wallet Balances:")
        for addr, wallet in self.processor.wallets.items():
            logger.info(f"{self.header}     - {wallet}")
        logger.info(self.header + " 📊 RevenueBot Auction State:")
        if self.revenue_bot.current_auction:
            for sp, amount in self.revenue_bot.current_auction.items():
                logger.info(f"{self.header}     - SP {sp} → {amount:.2f} FIL")
        else:
            logger.info(f"{self.header}     - ✅ No active contributors. Auction cleared.")
        logger.info(self.header + " " + "=" * 80 + '\n')