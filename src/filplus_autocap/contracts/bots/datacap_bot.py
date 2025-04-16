from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.contracts.bots.bot import Bot
from filplus_autocap.blockchain_utils.currencies import FIL, DAT


class DatacapBot(Bot):
    def __init__(self, address: str, datacap_wallet: Wallet):
        super().__init__(address)
        self.datacap_wallet = datacap_wallet

    def get_datacap_balance(self):
        return self.datacap_wallet.datacap_balance

    def create_datacap_tx(self, recipient_address: str, datacap_amount: DAT):
        tx = Tx(
            sender=self.datacap_wallet.address,
            recipient=recipient_address,
            datacap_amount=DAT(datacap_amount),
            fil_amount=FIL(0.0),
            signers=[],
        )
        self.sign_tx(tx)
        return tx