from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.utils.constants import DATACAP_MAX_ISSUANCE

class Filecoin(Wallet):
    def __init__(self, address: str):
        super().__init__(address=address, owner="filecoin")
        self.datacap_balance = DATACAP_MAX_ISSUANCE
    
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
            signers=signers,
            message=f"Mint {amount} DC to {recipient_wallet.address}"
        )

        return tx
