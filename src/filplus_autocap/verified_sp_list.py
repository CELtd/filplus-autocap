# src/filplus_autocap/verified_sp_list.py

class VerifiedSPList:
    def __init__(self):
        self.verified_addresses = set()

    def is_verified(self, address: str) -> bool:
        return address in self.verified_addresses

    def process_tx(self, tx):
        """
        Registers the sender of a zero-value tx as a verified SP.
        """
        self.verified_addresses.add(tx.sender)

    def __repr__(self):
        return f"<VerifiedSPList {len(self.verified_addresses)} addresses>"
