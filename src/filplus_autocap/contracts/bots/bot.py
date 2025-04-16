from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.blockchain_utils.wallet import Wallet

class Bot(Wallet):
    """
    A class representing a Bot, which extends the Wallet class. 
    The Bot can sign transactions by appending its address to the list of signers.
    """

    def __init__(self, address: str, owner: str = "bot"):
        """
        Initializes a new Bot object with the given address and an optional owner.
        
        Args:
            address (str): The address associated with the Bot.
            owner (str): The owner of the Bot (default is "bot").
        """
        # Calls the parent constructor (Wallet) to initialize the Bot with the address and owner.
        super().__init__(address=address, owner=owner)

    def sign_tx(self, tx: Tx):
        """
        Signs the transaction by appending the Bot's address to the list of signers.
        
        Args:
            tx (Tx): The transaction to sign.
        
        Returns:
            Tx: The modified transaction with the Bot's address added to the list of signers.
        """
        # Append the Bot's address to the list of signers in the transaction
        tx.signers.append(self.address)
        
        # Return the modified transaction
        return tx
