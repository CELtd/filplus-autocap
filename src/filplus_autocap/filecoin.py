# src/filplus_autocap/filecoin.py

from filplus_autocap.transaction import Tx
from filplus_autocap.wallet import Wallet

class Filecoin:
    def __init__(self, address: str, gas_price: float = 1):
        self.address = address
        self._gas_price = gas_price

    def get_current_gas_price(self) -> float:
        return self._gas_price

    def issue_datacap(self, recipient_wallet: Wallet, amount: float, signers: list[str] = None) -> Tx:
        """
        Issues datacap to a given wallet.
        
        Parameters:
            recipient_wallet (Wallet): The wallet to receive datacap.
            amount (float): The amount of datacap to send.
            signers (list[str], optional): List of signer addresses. Defaults to self.
        
        Returns:
            Tx: The transaction object representing the issuance.
        """
        if signers is None:
            signers = [self.address]

        tx = Tx(
            sender=self.address,
            recipient=recipient_wallet.address,
            datacap_amount=amount,
            fil_amount=0.0,
            signers=signers
        )

        recipient_wallet.datacap_balance += amount
        return tx
