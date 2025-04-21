from filplus_autocap.contracts.bots.bot import Bot
from filplus_autocap.blockchain_utils.transaction import Tx
from filplus_autocap.contracts.verified_sp_list import VerifiedSPList
from filplus_autocap.utils.constants import GAS_PRICE
from filplus_autocap.blockchain_utils.currencies import FIL, DAT

class RevenueBot(Bot):
    """
    The RevenueBot is responsible for managing the FIL revenue contributed by verified Storage Providers (SPs).
    It ensures that revenue from unverified SPs is redirected to the protocol wallet and tracks the contributions 
    of verified SPs in the current auction.
    """

    def __init__(
        self,
        address: str,
        protocol_wallet_address: str,
        verified_sp_list: VerifiedSPList,
    ):
        """
        Initializes the RevenueBot with the provided address, protocol wallet address, and verified SP list.
        
        Args:
            address (str): The address of the RevenueBot.
            protocol_wallet_address (str): The address of the protocol wallet to redirect unverified SP contributions.
            verified_sp_list (VerifiedSPList): The list of verified SP addresses.
        """
        # Initialize the Bot superclass with the provided address and owner details
        super().__init__(address=address, owner=["revenue_bot", "master_bot"])
        
        # Store the protocol wallet address to redirect revenue if needed
        self.protocol_wallet_address = protocol_wallet_address
        
        # Store the verified SP list to check if a sender is a verified SP
        self.verified_sp_list = verified_sp_list
        
        # Initialize a dictionary to track FIL contributions per verified SP in the current auction
        self.current_auction = {}

        # List to store outgoing transaction from non-verified SPs
        self.outgoing_txs = []

    def process_incoming_tx(self, tx: Tx) -> list[Tx]:
        """
        Processes an incoming transaction. If the sender is verified, it tracks their FIL contribution.
        Otherwise, it redirects the revenue to the protocol wallet.

        Args:
            tx (Tx): The transaction to process, which includes the sender and the FIL amount.

        Returns:
            list[Tx]: A list of outgoing transactions that either redirect the revenue or 
                      record the contribution of a verified SP.
        """
        # Extract the sender and FIL amount from the incoming transaction
        sender = tx.sender
        fil_amount = tx.fil_amount
        
        # Check if the sender is a verified SP
        is_verified = self.verified_sp_list.is_verified(sender)
        
        if not is_verified:
            # If the sender is unverified, redirect their revenue to the protocol wallet
            protocol_tx = Tx(
                sender=self.address,  # Sender is the RevenueBot
                recipient=self.protocol_wallet_address,  # Redirect to the protocol wallet
                datacap_amount=DAT(0.0),  # No Datacap involved in this transaction
                fil_amount=FIL(fil_amount),  # Transfer the FIL amount
                signers=[self.address],  # The RevenueBot signs the transaction
                message=f"Redirected revenue from unverified SP {sender}"  # Add a message for tracking
            )
            self.outgoing_txs.append(protocol_tx)  # Add the protocol transaction to the outgoing list
        else:
            # If the sender is verified, track their FIL contribution in the current auction
            self.current_auction[sender] = self.current_auction.get(sender, FIL(0.0)) + FIL(fil_amount)
            self.outgoing_txs.append(tx)

        # Return the list of outgoing transactions (could be empty or contain the protocol redirection)
        return self.outgoing_txs

    def drain_auction(self) -> dict[str, float]:
        """
        Drains the current auction by returning a copy of the tracked contributions and clearing the auction data.
        
        Returns:
            dict[str, float]: A dictionary containing the tracked FIL contributions per verified SP.
        """
        # Copy the current auction contributions to avoid data loss when clearing
        drained = self.current_auction.copy()
        
        # Clear the current auction to reset the tracker for the next round
        self.current_auction.clear()
        
        # Return the drained auction data
        return drained

    def __repr__(self):
        """
        Returns a string representation of the RevenueBot, including information from the superclass.
        
        Returns:
            str: The string representation of the RevenueBot.
        """
        # Call the superclass __repr__ method to get the basic wallet info and add more details if needed
        wallet_info = super().__repr__()
        return f"{wallet_info}"  # Return the representation, currently adding nothing new to the base info
