# src/filplus_autocap/storage_provider.py

from filplus_autocap.wallet import Wallet
from filplus_autocap.transaction import Tx


class StorageProvider:
    def __init__(self, name: str, wallet: Wallet):
        self.name = name
        self.wallet = wallet

    def send_fil(self, recipient_address: str, amount: float, signers: list[str]) -> Tx:
        if self.wallet.fil_balance < amount:
            raise ValueError(f"[{self.name}] Insufficient FIL balance to send {amount} FIL.")
        
        # Create a new transaction
        tx = Tx(
            sender_address=self.wallet.address,
            recipient_address=recipient_address,
            datacap_amount=0.0,
            fil_amount=amount,
            signers=signers
        )

        # Deduct the FIL from the wallet
        self.wallet.fil_balance -= amount

        return tx

    def __repr__(self):
        return f"<StorageProvider {self.name} | Wallet: {self.wallet.address}>"
