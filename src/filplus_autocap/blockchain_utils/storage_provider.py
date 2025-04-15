from filplus_autocap.blockchain_utils.wallet import Wallet

def initialize_sp(address: str, owner: str, fil_balance: float, datacap_balance: float = 0.0, wallets: dict = None, processor=None) -> Wallet:
    """
    Initializes a Storage Provider wallet and registers it in the given wallets dict and processor.

    Args:
        address (str): The Filecoin wallet address of the SP.
        owner (str): The owner label.
        fil_balance (float): FIL balance to initialize with.
        datacap_balance (float): Optional datacap balance.
        wallets (dict): Wallet dictionary to update (can be None).
        processor (TxProcessor): Processor to sync wallets with (can be None).

    Returns:
        Wallet: The initialized Wallet object.
    """
    sp_wallet = Wallet(
        address=address,
        owner=owner,
        fil_balance=fil_balance,
        datacap_balance=datacap_balance
    )

    if wallets is not None:
        wallets[address] = sp_wallet

    if processor is not None:
        processor.wallets = wallets

    return sp_wallet
