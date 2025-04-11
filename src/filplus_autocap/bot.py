from filplus_autocap.transaction import Tx

class Bot:
    def __init__(self, address: str):
        self.address = address

    def sign_tx(self, tx: Tx):
        tx.signers.append(self.address)
        return tx