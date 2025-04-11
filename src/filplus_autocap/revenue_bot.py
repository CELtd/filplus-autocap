from filplus_autocap.bot import Bot
from filplus_autocap.transaction import Tx
from filplus_autocap.verified_sp_list import VerifiedSPList
from filplus_autocap.constants import GAS_PRICE


class RevenueBot(Bot):
    def __init__(
        self,
        address: str,
        protocol_wallet_address: str,
        verified_sp_list: VerifiedSPList,
    ):
        super().__init__(address)
        self.protocol_wallet_address = protocol_wallet_address
        self.verified_sp_list = verified_sp_list
        self.current_auction = {}  # Tracks FIL contributions per verified SP

    def process_incoming_tx(self, tx: Tx) -> list[Tx]:
        sender = tx.sender
        fil_amount = tx.fil_amount
        outgoing_txs = []

        is_verified = self.verified_sp_list.is_verified(sender)
        if not is_verified:
            # Redirect revenue to protocol wallet if SP is unverified
            protocol_tx = Tx(
                sender=self.address,
                recipient=self.protocol_wallet_address,
                datacap_amount=0.0,
                fil_amount=fil_amount,
                signers=[self.address],
                message=f"Redirected revenue from unverified SP {sender}"
            )
            outgoing_txs.append(protocol_tx)
        else:
            # Record verified SP contribution in current auction
            self.current_auction[sender] = self.current_auction.get(sender, 0.0) + fil_amount

        return outgoing_txs

    def drain_auction(self) -> dict[str, float]:
        drained = self.current_auction.copy()
        self.current_auction.clear()
        return drained

    def __repr__(self):
        return f"<RevenueBot at {self.address} with {len(self.current_auction)} active contributors and a total of {self.fil_balance} FIL>"
