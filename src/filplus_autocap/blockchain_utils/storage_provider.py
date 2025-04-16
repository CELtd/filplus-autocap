from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.currencies import DAT, FIL

def initialize_sp(address: str, owner: str, fil_balance: FIL = FIL(0), datacap_balance: DAT = DAT(0.0), wallets: dict = None, processor=None) -> Wallet:
    """
    Initializes a Storage Provider (SP) wallet with a specified address, owner, and balances.
    Optionally registers the wallet in a given wallets dictionary and syncs with a processor.

    Args:
        address (str): The Filecoin wallet address of the Storage Provider (SP).
        owner (str): The owner label for the SP wallet.
        fil_balance (FIL, optional): The initial FIL balance to assign to the wallet. Default is 0 FIL.
        datacap_balance (DAT, optional): The initial datacap balance to assign to the wallet. Default is 0 DAT.
        wallets (dict, optional): A dictionary of wallets to update. If provided, the SP wallet will be added to it.
        processor (optional): A processor (e.g., a transaction processor) that will be synced with the wallets dictionary.

    Returns:
        Wallet: The initialized Wallet object representing the SP.

    Notes:
        - If the `wallets` dictionary is provided, the new SP wallet will be added to it with the `address` as the key.
        - If the `processor` is provided, the wallets dictionary will be synced with the `processor.wallets` attribute.
    """
    # Create a new Wallet instance for the Storage Provider (SP) with the specified balances
    sp_wallet = Wallet(
        address=address,
        owner=owner,
        fil_balance=FIL(fil_balance),
        datacap_balance=DAT(datacap_balance)
    )

    # If a wallets dictionary is provided, add the newly created wallet to it
    if wallets is not None:
        wallets[address] = sp_wallet

    # If a processor is provided, sync the wallets with the processor
    if processor is not None:
        processor.wallets = wallets

    # Return the initialized wallet
    return sp_wallet
