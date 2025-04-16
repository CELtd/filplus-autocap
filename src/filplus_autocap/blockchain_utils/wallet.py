import re
from typing import Union, List
from filplus_autocap.blockchain_utils.currencies import DAT, FIL

class Wallet:
    """
    Represents a wallet that holds FIL and Datacap balances for a specific address.
    
    Attributes:
        address (str): The address of the wallet.
        owner (List[str]): A list of owners of the wallet. Can also accept a single string.
        datacap_balance (DAT): The balance of Datacap associated with the wallet.
        fil_balance (FIL): The balance of FIL associated with the wallet.
    """
    
    def __init__(
        self,
        address: str,
        owner: Union[str, List[str]],
        datacap_balance: DAT = DAT(0),
        fil_balance: FIL = FIL(0)
    ):
        """
        Initializes a new Wallet object with the provided address, owner, and balances.

        Args:
            address (str): The address associated with the wallet.
            owner (Union[str, List[str]]): A single owner or a list of owners of the wallet.
            datacap_balance (DAT, optional): The initial balance of Datacap. Defaults to 0.
            fil_balance (FIL, optional): The initial balance of FIL. Defaults to 0.
        """
        self.address = address
        self.owner = [owner] if isinstance(owner, str) else owner
        self.datacap_balance = DAT(datacap_balance)
        self.fil_balance = FIL(fil_balance)

    def deposit_datacap(self, amount: float):
        """
        Deposits Datacap into the wallet.

        Args:
            amount (float): The amount of Datacap to deposit.

        Raises:
            ValueError: If the deposit amount is negative.
        """
        if amount < 0:
            raise ValueError("Cannot deposit negative datacap.")
        self.datacap_balance += amount

    def withdraw_datacap(self, amount: float):
        """
        Withdraws Datacap from the wallet.

        Args:
            amount (float): The amount of Datacap to withdraw.

        Raises:
            ValueError: If the withdrawal amount exceeds the current balance.
        """
        if amount > self.datacap_balance:
            raise ValueError("Insufficient datacap.")
        self.datacap_balance -= amount

    def deposit_fil(self, amount: float):
        """
        Deposits FIL into the wallet.

        Args:
            amount (float): The amount of FIL to deposit.

        Raises:
            ValueError: If the deposit amount is negative.
        """
        if amount < 0:
            raise ValueError("Cannot deposit negative FIL.")
        self.fil_balance += amount

    def withdraw_fil(self, amount: float):
        """
        Withdraws FIL from the wallet.

        Args:
            amount (float): The amount of FIL to withdraw.

        Raises:
            ValueError: If the withdrawal amount exceeds the current balance.
        """
        if amount > self.fil_balance:
            raise ValueError("Insufficient FIL.")
        self.fil_balance -= amount

    def __repr__(self):
        """
        Provides a string representation of the Wallet object.

        Returns:
            str: A string that represents the Wallet object in the format:
                 '<Wallet address={address} owner={owner} datacap={datacap_balance} FIL={fil_balance}>'
        """
        return (
            f"<Wallet address={self.address} "
            f"owner={self.owner} "
            f"datacap={self.datacap_balance} "
            f"FIL={self.fil_balance}>"
        )
    
    @staticmethod
    def reconstruct_wallet_from_repr(wallet_repr: str) -> 'Wallet':
        """
        Reconstructs a Wallet object from its string representation or message format.
        
        This method extracts wallet details from a string in the format:
        '<Wallet address={address} owner={owner} datacap={datacap_balance} FIL={fil_balance}>'
        
        Args:
            wallet_repr (str): A string containing the wallet information in either the
                               wallet repr format or a message format, e.g.,
                               'Registering SP with wallet <Wallet address=f1_wallet ...>'
        
        Returns:
            Wallet: A Wallet object containing the extracted wallet details.

        Raises:
            ValueError: If the string format is invalid or cannot be parsed correctly.
        """
        # Extract wallet information from the provided string using regular expressions
        wallet_info_pattern = r"<Wallet address=(?P<address>.*?) owner=(?P<owner>.*?) datacap=(?P<datacap>.*?) FIL=(?P<fil>.*?)>"
        match = re.search(wallet_info_pattern, wallet_repr)
        
        if match:
            wallet_data = match.groupdict()
            
            # Extract and convert the relevant details to their appropriate types
            address = wallet_data['address']
            owner = eval(wallet_data['owner'])  # Convert the owner string to a list if necessary
            datacap_balance = DAT(float(wallet_data['datacap']))
            fil_balance = FIL(float(wallet_data['fil']))
            
            # Return the Wallet object with the extracted information
            return Wallet(address=address, owner=owner, datacap_balance=datacap_balance, fil_balance=fil_balance)
        else:
            raise ValueError("Invalid wallet information format in message or repr")
