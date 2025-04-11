from typing import Union, List


class Wallet:
    def __init__(
        self,
        address: str,
        owner: Union[str, List[str]],
        datacap_balance: float = 0.0,
        fil_balance: float = 0.0
    ):
        self.address = address
        self.owner = [owner] if isinstance(owner, str) else owner
        self.datacap_balance = datacap_balance
        self.fil_balance = fil_balance

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
            f"datacap={self.datacap_balance:.2f} "
            f"FIL={self.fil_balance:.2f}>"
        )
    
