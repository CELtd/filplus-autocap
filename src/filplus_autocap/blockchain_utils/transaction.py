import sys
from typing import List, Optional, TYPE_CHECKING
from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.currencies import FIL, DAT
from filplus_autocap.utils.constants import FILECOIN_ADDRESS

# TYPE_CHECKING import for forward references, primarily for type hinting
if TYPE_CHECKING:
    from filplus_autocap.contracts.verified_sp_list import VerifiedSPList


class Tx:
    """
    Represents a transaction between two Filecoin wallets, involving both FIL and Datacap amounts.

    Attributes:
        sender (str): The address of the sender of the transaction.
        recipient (str): The address of the recipient of the transaction.
        datacap_amount (DAT): The amount of Datacap to be transferred.
        fil_amount (FIL): The amount of FIL to be transferred.
        signers (List[str]): A list of signer addresses involved in the transaction.
        message (Optional[str]): An optional message associated with the transaction.
    """
    def __init__(
        self,
        sender: str,
        recipient: str,
        datacap_amount: DAT = DAT(0),
        fil_amount: FIL = FIL(0),
        signers: Optional[List[str]] = None,
        message: Optional[str] = None
    ):
        """
        Initializes a transaction with sender, recipient, Datacap, FIL amounts, signers, and an optional message.

        Args:
            sender (str): The address of the transaction sender.
            recipient (str): The address of the transaction recipient.
            datacap_amount (DAT, optional): The amount of Datacap to send. Defaults to 0 DAT.
            fil_amount (FIL, optional): The amount of FIL to send. Defaults to 0 FIL.
            signers (Optional[List[str]], optional): List of signer addresses. Defaults to an empty list if not provided.
            message (Optional[str], optional): A custom message for the transaction.
        """
        self.sender = sender
        self.recipient = recipient
        self.datacap_amount = DAT(datacap_amount)
        self.fil_amount = FIL(fil_amount)
        self.signers = signers or []
        self.message = message

    def __repr__(self):
        """
        Provides a string representation of the transaction for debugging.

        Returns:
            str: A string representation of the transaction.
        """
        return (
            f"<Transaction from={self.sender} to={self.recipient} "
            f"datacap={self.datacap_amount} FIL={self.fil_amount} "
            f"signers={self.signers} message={self.message!r}>"
        )


class TxProcessor:
    """
    Processes transactions by ensuring that the sender has sufficient balances
    and transferring FIL and Datacap between wallets.

    Attributes:
        wallets (dict): A dictionary mapping wallet addresses to Wallet objects.
    """
    def __init__(self, wallets: dict[str, Wallet]):
        """
        Initializes the transaction processor with a set of wallets.

        Args:
            wallets (dict): A dictionary where keys are wallet addresses and values are Wallet objects.
        """
        self.wallets = wallets

    def send(self, txs: List[Tx]):
        """
        Processes a list of transactions. Transfers FIL and Datacap between sender and recipient wallets.

        Args:
            txs (List[Tx]): A list of Tx objects to process.

        Raises:
            ValueError: If a recipient wallet is missing, if the sender wallet has insufficient balance,
                        or if any other error occurs during the transaction.
        """
        for tx in txs:
            # Retrieve the sender and recipient wallets from the wallets dictionary
            sender_wallet = self.wallets.get(tx.sender)
            recipient_wallet = self.wallets.get(tx.recipient)

            # Raise error if the recipient wallet is not found
            if recipient_wallet is None:
                raise ValueError(f"Missing recipient wallet in tx: {tx}")

            # If the recipient is a VerifiedSPList, process the transaction differently
            if recipient_wallet.__class__.__name__ == "VerifiedSPList":
                recipient_wallet.process_tx(tx)
                continue

            # Raise error if the sender wallet is not found
            if sender_wallet is None:
                raise ValueError(f"Missing sender wallet in tx: {tx}")

            # Check if the sender wallet has sufficient FIL balance
            if tx.sender != FILECOIN_ADDRESS and sender_wallet.fil_balance < tx.fil_amount:
                raise ValueError(
                    f"Insufficient FIL {sender_wallet.fil_balance} in sender wallet: {tx.sender}"
                )

            # Check if the sender wallet has sufficient Datacap balance
            if sender_wallet.datacap_balance < tx.datacap_amount:
                raise ValueError(
                    f"Insufficient Datacap {sender_wallet.datacap_balance} in sender wallet: {tx.sender}"
                )

            # Process FIL transfer between sender and recipient wallets
            sender_wallet.fil_balance -= tx.fil_amount
            recipient_wallet.fil_balance += tx.fil_amount

            # Process Datacap transfer between sender and recipient wallets
            sender_wallet.datacap_balance -= tx.datacap_amount
            recipient_wallet.datacap_balance += tx.datacap_amount
