import json
from pathlib import Path
from dataclasses import dataclass

from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.filecoin import Filecoin
from filplus_autocap.blockchain_utils.transaction import TxProcessor
from filplus_autocap.contracts.bots.revenue_bot import RevenueBot
from filplus_autocap.contracts.bots.master_bot import MasterBot
from filplus_autocap.contracts.bots.datacap_bot import DatacapBot
from filplus_autocap.contracts.verified_sp_list import VerifiedSPList
from filplus_autocap.utils.constants import GAS_PRICE, FILECOIN_ADDRESS, VERIFIED_SP_FILE
from filplus_autocap.blockchain_utils.currencies import DAT, FIL

@dataclass
class SetupEnv:
    """
    Environment setup for the system, encapsulating all necessary components.
    """
    wallets: dict[str, Wallet]
    processor: TxProcessor
    revenue_bot: RevenueBot
    master_bot: MasterBot
    datacap_bot: DatacapBot
    verified_list: VerifiedSPList

def load_config(path: str = "config/setup.json") -> dict:
    """
    Loads the configuration from a JSON file.

    Args:
        path (str): Path to the configuration file. Default is 'config/setup.json'.
    
    Returns:
        dict: Parsed JSON configuration.
    """
    with open(Path(path), "r") as f:
        return json.load(f)

def initialize(config_path: str = "config/setup.json") -> SetupEnv:
    """
    Initializes the system environment using the provided configuration.

    Args:
        config_path (str): Path to the configuration file. Default is 'config/setup.json'.
    
    Returns:
        SetupEnv: The initialized environment with all components.
    """
    # Load the configuration from the JSON file
    config = load_config(config_path)

    # Initialize wallets with predefined values
    datacap_wallet = Wallet(
        address="f1_datacap_wallet",
        owner=["datacap_bot", "master_bot"],
        datacap_balance=DAT(10_000)
    )
    protocol_wallet = Wallet(address="f1_protocol_wallet", owner="protocol")
    burn_wallet = Wallet(address="f099", owner="filecoin", fil_balance=FIL(0.0))
    filecoin = Filecoin(FILECOIN_ADDRESS)

    # Initialize the VerifiedSPList (Storage Providers)
    verified_list = VerifiedSPList()

    # Initialize the RevenueBot
    revenue_bot = RevenueBot(
        address="f1_revenue_bot",
        protocol_wallet_address=protocol_wallet.address,
        verified_sp_list=verified_list
    )

    # Initialize the DatacapBot
    datacap_bot = DatacapBot(
        address="f1_datacap_bot",
        datacap_wallet=datacap_wallet
    )

    # Initialize the MasterBot with the necessary configuration parameters
    master_bot = MasterBot(
        address="f1_master_bot",
        revenue_bot=revenue_bot,
        datacap_bot=datacap_bot,
        master_fee_ratio=FIL(config["auction_fee"]),
        datacap_distribution_round=DAT(config["datacap_per_round"]),
        auction_duration=config["auction_duration"]
    )

    # Create a dictionary of all the wallets
    wallets = {
        "f_filecoin": filecoin,
        "f1_datacap_wallet": datacap_wallet,
        "f1_protocol_wallet": protocol_wallet,
        "f1_revenue_bot": revenue_bot,
        "f_verifiedsp_list": verified_list,
        "f1_master_bot": Wallet(address="f1_master_bot", owner="master_bot"),
        "f099": burn_wallet,
    }

    # Initialize the transaction processor with the wallets
    processor = TxProcessor(wallets)

    # Load and initialize the verified SP addresses into the verified list
    verified_list.load(processor)

    # Assign the processor to the master bot
    master_bot.processor = processor

    # Return the complete environment setup
    return SetupEnv(
        wallets=wallets,
        processor=processor,
        revenue_bot=revenue_bot,
        master_bot=master_bot,
        datacap_bot=datacap_bot,
        verified_list=verified_list,
    )
