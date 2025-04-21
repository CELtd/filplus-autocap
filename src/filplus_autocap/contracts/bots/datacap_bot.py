from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.contracts.bots.bot import Bot
from filplus_autocap.blockchain_utils.currencies import FIL, DAT

class DatacapBot(Bot):
    """
    A class representing a DatacapBot, which extends the Bot class. 
    The DatacapBot is responsible for handling transactions involving Datacap.
    It has access to a specific `datacap_wallet` and is used for creating and sending Datacap transactions.
    """

    def __init__(self, address: str, datacap_wallet: Wallet):
        """
        Initializes the DatacapBot object with the given address and a specific datacap wallet.
        
        Args:
            address (str): The address associated with the DatacapBot.
            datacap_wallet (Wallet): The wallet that contains the Datacap balance.
        """
        # Initialize the DatacapBot by calling the constructor of the Bot class
        super().__init__(address)
        
        # Set the datacap_wallet property to the provided Wallet instance
        self.datacap_wallet = datacap_wallet

    def get_datacap_balance(self):
        """
        Retrieves the current Datacap balance from the datacap_wallet.
        
        Returns:
            DAT: The current Datacap balance in the datacap_wallet.
        """
        # Return the Datacap balance of the associated datacap_wallet
        return self.datacap_wallet.datacap_balance

    def create_datacap_tx(self, recipient_address: str, datacap_amount: DAT, message : str = "DAT issued"):
        """
        Creates a transaction for transferring Datacap from the DatacapBot's datacap_wallet.
        
        Args:
            recipient_address (str): The address to send Datacap to.
            datacap_amount (DAT): The amount of Datacap to send.
        
        Returns:
            Tx: The generated transaction that is signed by the DatacapBot.
        """
        # Create a new transaction where:
        # - The sender is the datacap_wallet's address
        # - The recipient is the specified recipient address
        # - The Datacap amount is specified, and no FIL is involved in this transaction
        tx = Tx(
            sender=self.datacap_wallet.address,  # Sender is the datacap_wallet's address
            recipient=recipient_address,  # Recipient is the address passed in
            datacap_amount=DAT(datacap_amount),  # Datacap amount to transfer
            fil_amount=FIL(0.0),  # No FIL is involved in this transaction
            signers=[],  # The list of signers is initially empty
            message=message
        )
        
        # Return the signed transaction
        return self.sign_tx(tx)
