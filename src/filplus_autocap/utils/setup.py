from dataclasses import dataclass

from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.filecoin import Filecoin
from filplus_autocap.blockchain_utils.transaction import TxProcessor
from filplus_autocap.contracts.bots.revenue_bot import RevenueBot
from filplus_autocap.contracts.bots.master_bot import MasterBot
from filplus_autocap.contracts.bots.datacap_bot import DatacapBot
from filplus_autocap.contracts.verified_sp_list import VerifiedSPList
from filplus_autocap.utils.constants import GAS_PRICE, FILECOIN_ADDRESS


@dataclass
class SetupEnv:
    wallets: dict[str, Wallet]
    processor: TxProcessor
    revenue_bot: RevenueBot
    master_bot: MasterBot
    datacap_bot: DatacapBot
    verified_list: VerifiedSPList

def initialize() -> SetupEnv:
    # Wallets and bot setup
    datacap_wallet = Wallet(address="f1_datacap_wallet", owner="datacap-bot", datacap_balance=10_000)
    protocol_wallet = Wallet(address="f1protocolwallet", owner="protocol")
    burn_wallet = Wallet(address="f099", owner="burn", fil_balance=0.0)
    filecoin = Filecoin(FILECOIN_ADDRESS)

    verified_list = VerifiedSPList()
    revenue_bot = RevenueBot(address="f1revenuebot", protocol_wallet_address=protocol_wallet.address, verified_sp_list=verified_list)
    datacap_bot = DatacapBot(address="f1datacapbot", datacap_wallet=datacap_wallet)
    master_bot = MasterBot(
        address="f1masterbot",
        revenue_bot=revenue_bot,
        datacap_bot=datacap_bot,
        master_fee_ratio=0.1,
        datacap_distribution_total=1000.0
    )

    wallets = {
        "f_filecoin": filecoin,
        "f1_datacap_wallet": datacap_wallet,
        "f1protocolwallet": protocol_wallet,
        "f1revenuebot": revenue_bot,
        "f1datacapbot": datacap_wallet,
        "f_verifiedsp_list": verified_list,
        "f1masterbot": Wallet(address="f1masterbot", owner="master-bot"),  # dummy for TxProcessor
        "f099": burn_wallet,
    }

    processor = TxProcessor(wallets)

    return SetupEnv(
        wallets=wallets,
        processor=processor,
        revenue_bot=revenue_bot,
        master_bot=master_bot,
        datacap_bot=datacap_bot,
        verified_list=verified_list,
    )
