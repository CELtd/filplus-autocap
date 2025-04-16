import re
from typing import Union, List
from filplus_autocap.blockchain_utils.currencies import DAT, FIL

class Wallet:
    def __init__(
        self,
        address: str,
        owner: Union[str, List[str]],
        datacap_balance: DAT = DAT(0),
        fil_balance: FIL = FIL(0)
    ):
        self.address = address
        self.owner = [owner] if isinstance(owner, str) else owner
        self.datacap_balance = DAT(datacap_balance)
        self.fil_balance = FIL(fil_balance)

    def deposit_datacap(self, amount: float):
        if amount < 0:
            raise ValueError("Cannot deposit negative datacap.")
        self.datacap_balance += amount

    def withdraw_datacap(self, amount: float):
        if amount > self.datacap_balance:
            raise ValueError("Insufficient datacap.")
        self.datacap_balance -= amount

    def deposit_fil(self, amount: float):
        if amount < 0:
            raise ValueError("Cannot deposit negative FIL.")
        self.fil_balance += amount

    def withdraw_fil(self, amount: float):
        if amount > self.fil_balance:
            raise ValueError("Insufficient FIL.")
        self.fil_balance -= amount

    def __repr__(self):
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
        
        Args:
            wallet_repr (str): A string containing the wallet information in either the
                               wallet repr format or the message format, e.g.,
                               'Registering SP with wallet <Wallet address=f1_wallet ...>'
        
        Returns:
            Wallet: A Wallet object containing the extracted wallet details.
        """
        # Extract wallet information from message or repr format
        wallet_info_pattern = r"<Wallet address=(?P<address>.*?) owner=(?P<owner>.*?) datacap=(?P<datacap>.*?) FIL=(?P<fil>.*?)>"
        match = re.search(wallet_info_pattern, wallet_repr)
        
        if match:
            wallet_data = match.groupdict()
            
            # Extract and convert the relevant details
            address = wallet_data['address']
            owner = eval(wallet_data['owner'])  # Convert the owner string to a list if necessary
            datacap_balance = DAT(float(wallet_data['datacap']))
            fil_balance = FIL(float(wallet_data['fil']))
            
            # Return the Wallet object with the extracted information
            return Wallet(address=address, owner=owner, datacap_balance=datacap_balance, fil_balance=fil_balance)
        else:
            raise ValueError("Invalid wallet information format in message or repr")
