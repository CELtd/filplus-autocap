from wallet import Wallet
from transaction import Tx
from bot import Bot


class DatacapBot(Bot):
    def __init__(self, address: str, datacap_wallet: Wallet):
        super().__init__(address)
        self.datacap_wallet = datacap_wallet

    def get_datacap_balance(self):
        return self.datacap_wallet.datacap

    def create_allocation_tx(self, recipient_address: str, datacap_amount: float, master_address: str):
        tx = Tx(
            sender=self.datacap_wallet.address,
            recipient=recipient_address,
            datacap=datacap_amount,
            fil=0.0,
            signers=[]
        )
        self.sign_transaction(tx)
        tx.signers.append(master_address)
        return tx