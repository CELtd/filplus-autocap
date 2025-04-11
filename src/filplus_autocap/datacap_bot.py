from filplus_autocap.wallet import Wallet
from filplus_autocap.transaction import Tx
from filplus_autocap.bot import Bot


class DatacapBot(Bot):
    def __init__(self, address: str, datacap_wallet: Wallet):
        super().__init__(address)
        self.datacap_wallet = datacap_wallet

    def get_datacap_balance(self):
        return self.datacap_wallet.datacap_balance

    def create_datacap_tx(self, recipient_address: str, datacap_amount: float):
        tx = Tx(
            sender=self.datacap_wallet.address,
            recipient=recipient_address,
            datacap_amount=datacap_amount,
            fil_amount=0.0,
            signers=[],
        )
        self.sign_tx(tx)
        return tx