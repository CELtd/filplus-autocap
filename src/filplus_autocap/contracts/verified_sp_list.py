import sys
import json
from pathlib import Path
from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.transaction import Tx, TxProcessor
from filplus_autocap.utils.constants import VERIFIED_SP_FILE

class VerifiedSPList(Wallet):
    def __init__(self, address: str = "f_verifiedsp_list", filepath: Path = Path(VERIFIED_SP_FILE), processor: TxProcessor = None):
        """
        Initializes the VerifiedSPList with the given address, filepath, and transaction processor.
        
        Args:
            address (str): The address of the VerifiedSPList wallet.
            filepath (Path): Path to the JSON file storing verified SP data.
            processor (TxProcessor): The transaction processor for managing wallet transactions.
        """
        super().__init__(address=address, owner="VerifiedSPList")
        self.filepath = filepath
        self.verified_wallets = {}  # Dictionary to store verified wallets, keyed by address.
        self.load(processor)  # Load existing verified SPs from file.

    def is_verified(self, address: str) -> bool:
        """
        Checks if the given address is a verified SP.
        
        Args:
            address (str): The address to check.
        
        Returns:
            bool: True if the address is verified, False otherwise.
        """
        return address in self.verified_wallets

    def process_tx(self, tx: Tx):
        """
        Registers the sender of a zero-value transaction directed to this address as a verified SP.
        
        Args:
            tx (Tx): The transaction to process.
        """
        if tx.recipient == self.address:
            try:
                # Reconstruct wallet from the transaction message
                self.verified_wallets[tx.sender] = Wallet.reconstruct_wallet_from_repr(tx.message)
                self.save()  # Save the updated list of verified wallets.
            except Exception as e:
                sys.stderr.write(f"Error processing transaction: {e}\n")
                # You can choose to log this error as well for further analysis.

    def load(self, processor: TxProcessor = None):
        """
        Loads the verified addresses and wallet data from a JSON file.
        
        Args:
            processor (TxProcessor, optional): The processor for adding wallets. Defaults to None.
        """
        if self.filepath.exists():
            with open(self.filepath, "r") as f:
                verified_data = json.load(f)
                # Reconstruct wallets from the JSON data and assign them to the verified wallets list
                for address_data in verified_data:
                    wallet_repr = address_data.get("repr")
                    if wallet_repr:
                        address = address_data["address"]
                        wallet = Wallet.reconstruct_wallet_from_repr(wallet_repr)
                        self.verified_wallets[address] = wallet
                        
                        # If a processor is provided, add the wallet to it
                        if processor:
                            processor.wallets[address] = wallet

    def save(self):
        """
        Saves the wallet information of verified addresses to a JSON file.
        Ensures each address is unique and overwrites existing entries.
        
        Updates:
            - Each wallet is assigned a unique number.
            - Existing entries are overwritten, and new ones are added.
        """
        verified_data = []
        for i, (address, wallet) in enumerate(self.verified_wallets.items(), start=1):
            wallet_data = {
                "number": i,  # Assign a unique number to each wallet.
                "address": wallet.address,  # Save the address.
                "repr": repr(wallet)  # Save the string representation of the wallet.
            }

            # Check if the address already exists, if so, overwrite it.
            existing_wallet_index = next((index for index, item in enumerate(verified_data) if item["address"] == address), None)
            if existing_wallet_index is not None:
                verified_data[existing_wallet_index] = wallet_data
            else:
                verified_data.append(wallet_data)
        
        # Write the updated list of verified wallets to the file
        with open(self.filepath, "w") as f:
            json.dump(verified_data, f, indent=2)

    def __repr__(self):
        return f"<VerifiedSPList {len(self.verified_wallets)} wallets: {list(self.verified_wallets.keys())}>"
