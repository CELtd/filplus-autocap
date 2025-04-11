from filplus_autocap.transaction import Tx
from filplus_autocap.wallet import Wallet

class Bot(Wallet):
    def __init__(self, address: str, owner: str = "bot"):
        super().__init__(address=address, owner=owner)

    def sign_tx(self, tx: Tx):
        tx.signers.append(self.address)
        return tx