import sys
import json
from pathlib import Path
from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.transaction import Tx, TxProcessor
from filplus_autocap.utils.constants import VERIFIED_SP_FILE

class VerifiedSPList(Wallet):
    def __init__(self, address: str = "f_verifiedsp_list", filepath: Path = Path(VERIFIED_SP_FILE), processor: TxProcessor = None):
        super().__init__(address=address, owner="VerifiedSPList")
        self.filepath = filepath
        self.verified_wallets = {}  # Keyed by address
        self.load(processor)

    def is_verified(self, address: str) -> bool:
        return address in self.verified_wallets

    def process_tx(self, tx: Tx):
        """
        Registers the sender of a zero-value tx directed to this address as a verified SP.
        """
        if tx.recipient == self.address:
            # Extract wallet information from the message and reconstruct the Wallet object
            self.verified_wallets[tx.sender] = Wallet.reconstruct_wallet_from_repr(tx.message)
            self.save()  # Persist the updated wallet list

    def load(self, processor: TxProcessor = None):
        """
        Loads the verified addresses and wallet data from a JSON file.
        """
        if self.filepath.exists():
            with open(self.filepath, "r") as f:
                verified_data = json.load(f)
                # Reconstruct wallets for each verified address
                for address_data in verified_data:
                    wallet_repr = address_data.get("repr")
                    if wallet_repr:
                        address = address_data["address"]
                        wallet = Wallet.reconstruct_wallet_from_repr(wallet_repr)
                        self.verified_wallets[address] = wallet
                        
                        # Ensure processor exists and assign the wallet to it
                        if processor:
                            processor.wallets[address] = wallet

    def save(self):
        """
        Saves the wallet information of verified addresses to a JSON file.
        Ensures each address is unique and overwrites existing entries.
        Assigns a number to each verified wallet.
        """
        # Prepare the data with numbers and updated wallet information
        verified_data = []
        for i, (address, wallet) in enumerate(self.verified_wallets.items(), start=1):
            wallet_data = {
                "number": i,  # Assign a unique number to each wallet
                "address": wallet.address,  # Save the address
                "repr": repr(wallet)  # Save the repr of the wallet
            }

            # Check if the address is already in the data, if so, overwrite it
            existing_wallet_index = next((index for index, item in enumerate(verified_data) if item["address"] == address), None)
            if existing_wallet_index is not None:
                # Overwrite existing entry
                verified_data[existing_wallet_index] = wallet_data
            else:
                # Add new wallet data
                verified_data.append(wallet_data)
        
        # Write the updated wallet list to the JSON file
        with open(self.filepath, "w") as f:
            json.dump(verified_data, f, indent=2)

    def __repr__(self):
        return f"<VerifiedSPList {len(self.verified_wallets)} wallets: {list(self.verified_wallets.keys())}>"
