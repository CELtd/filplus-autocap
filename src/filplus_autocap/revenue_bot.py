from filplus_autocap.bot import Bot
from filplus_autocap.transaction import Tx
from filplus_autocap.verified_sp_list import VerifiedSPList
from filplus_autocap.constants import GAS_PRICE


class RevenueBot(Bot):
    def __init__(
        self,
        address: str,
        revenue_wallet_address: str,
        info_vault_address: str,
        protocol_wallet_address: str,
        verified_sp_list: VerifiedSPList,
    ):
        super().__init__(address)
        self.revenue_wallet_address = revenue_wallet_address
        self.info_vault_address = info_vault_address
        self.protocol_wallet_address = protocol_wallet_address
        self.verified_sp_list = verified_sp_list

    def process_incoming_tx(self, tx: Tx) -> list[Tx]:
        sender = tx.sender
        fil_amount = tx.fil_amount
        outgoing_txs = []

        # Check if sender is verified
        is_verified = self.verified_sp_list.is_verified(sender)
        if not is_verified:
            registration_fee = 5 * GAS_PRICE
            protocol_fee =  GAS_PRICE

            if fil_amount < registration_fee:
                raise ValueError(f"Sender {sender} does not have enough FIL to register (requires {registration_fee})")

            fil_amount -= registration_fee

            # Bot emits a registration transaction to mark the sender as verified
            registration_tx = Tx(
                sender=self.address,
                recipient=sender,
                datacap_amount=0.0,
                fil_amount=0.0,
                signers=[self.address],
                message=f"SP registered"
            )
            outgoing_txs.append(registration_tx)

            # Send protocol fee (gas_price)
            protocol_tx = Tx(
                sender=self.address,
                recipient=self.protocol_wallet_address,
                datacap_amount=0.0,
                fil_amount=protocol_fee,
                signers=[self.address]
            )
            outgoing_txs.append(protocol_tx)

        # Forward SP revenue info to info vault
        info_tx = Tx(
            sender=self.address,
            recipient=self.info_vault_address,
            datacap_amount=0.0,
            fil_amount=0.0,
            signers=[self.address],
            message=f"Revenue received from {sender}: {fil_amount} FIL"
        )
        outgoing_txs.append(info_tx)

        # Transfer FIL to revenue wallet
        revenue_tx = Tx(
            sender=self.address,
            recipient=self.revenue_wallet_address,
            datacap_amount=0.0,
            fil_amount=fil_amount,
            signers=[self.address]
        )
        outgoing_txs.append(revenue_tx)

        return outgoing_txs
