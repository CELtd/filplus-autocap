from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.blockchain_utils.currencies import DAT, FIL
from filplus_autocap.utils.constants import DATACAP_MAX_ISSUANCE


class Filecoin(Wallet):
    """
    A class representing a Filecoin wallet with the capability to issue datacap.

    Inherits from the `Wallet` class and allows issuing datacap to other wallets.
    This class is designed to manage the datacap balance and facilitate datacap issuance.

    Attributes:
        datacap_balance (DAT): The balance of datacap available for issuance.
    """
    
    def __init__(self, address: str):
        """
        Initializes a Filecoin wallet instance.

        Args:
            address (str): The address of the Filecoin wallet.
        """
        # Call the constructor of the parent Wallet class with the address and owner set to "filecoin"
        super().__init__(address=address, owner="filecoin")
        # Set the initial datacap balance to the maximum allowed issuance
        self.datacap_balance = DAT(DATACAP_MAX_ISSUANCE)

    def issue_datacap(self, recipient_wallet: Wallet, amount: DAT = DAT(0), signers: list[str] = None) -> Tx:
        """
        Issues a specified amount of datacap to a given recipient wallet.

        Args:
            recipient_wallet (Wallet): The wallet that will receive the datacap.
            amount (DAT): The amount of datacap to be issued (default is 0).
            signers (list[str], optional): A list of signers' addresses. Defaults to the current wallet address.

        Returns:
            Tx: A transaction object representing the datacap issuance.
        """
        # If no signers are provided, use the address of this wallet as the default signer
        if signers is None:
            signers = [self.address]

        # Create the transaction to issue the specified amount of datacap
        tx = Tx(
            sender=self.address,  # Sender's wallet address
            recipient=recipient_wallet.address,  # Recipient's wallet address
            datacap_amount=amount,  # Amount of datacap being issued
            fil_amount=FIL(0),  # No FIL is being transferred in this transaction
            signers=signers,  # List of signers
            message=f"Mint {amount} DC to {recipient_wallet.address}"  # Transaction message
        )

        return tx
