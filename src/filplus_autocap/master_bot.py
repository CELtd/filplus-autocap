from filplus_autocap.transaction import Tx
from filplus_autocap.revenue_bot import RevenueBot
from filplus_autocap.datacap_bot import DatacapBot

class MasterBot:
    def __init__(
        self,
        address: str,
        revenue_bot: RevenueBot,
        datacap_bot: DatacapBot,
        master_fee_ratio: float = 0.1,
        protocol_fee_ratio: float = 0.1,
        datacap_distribution_total: float = 1000.0,
        protocol_wallet_address: str = "f1protocolwallet",
        burn_address: str = "f099",
    ):
        self.address = address
        self.revenue_bot = revenue_bot
        self.datacap_bot = datacap_bot
        self.master_fee_ratio = master_fee_ratio
        self.protocol_fee_ratio = protocol_fee_ratio
        self.datacap_distribution_total = datacap_distribution_total
        self.protocol_wallet_address = protocol_wallet_address
        self.burn_address = burn_address

    def execute_auction_round(self) -> list[Tx]:
        auction_data = self.revenue_bot.drain_auction()
        total_fil = sum(auction_data.values())
        reward_txs = []

        if total_fil == 0:
            return []

        for sp_address, contribution in auction_data.items():
            c_i = contribution / total_fil
            refund_amount = (1 - self.master_fee_ratio) * contribution
            datacap_amount = c_i * self.datacap_distribution_total
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

        # Compute and add burn + fee txs
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