from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.transaction import Tx

class VerifiedSPList(Wallet):
    def __init__(self, address: str = "f_verifiedsp_list"):
        super().__init__(address=address, owner="VerifiedSPList")
        self.verified_addresses = set()

    def is_verified(self, address: str) -> bool:
        return address in self.verified_addresses

    def process_tx(self, tx: Tx):
        """
        Registers the sender of a zero-value tx directed to this address as a verified SP.
        """
        if tx.recipient == self.address and tx.fil_amount == 0 and tx.datacap_amount == 0:
            self.verified_addresses.add(tx.sender)

    def __repr__(self):
        return f"<VerifiedSPList {len(self.verified_addresses)} addresses: {list(self.verified_addresses)}>"