from filplus_autocap.filecoin import Filecoin
from filplus_autocap.wallet import Wallet
from filplus_autocap.revenue_bot import RevenueBot
from filplus_autocap.verified_sp_list import VerifiedSPList
from filplus_autocap.transaction import Tx, TxProcessor
from filplus_autocap.constants import GAS_PRICE

# Setup wallets
datacap_wallet = Wallet(address="f_datacap_wallet", owner="datacap-bot")
sp_wallet = Wallet(address="f1sp001", owner="sp001", fil_balance=100.0 + GAS_PRICE)
revenue_wallet = Wallet(address="f1revenuewallet", owner="revenue-wallet")
info_vault = Wallet(address="f1vaultinfo", owner="vault")
protocol_wallet = Wallet(address="f1protocolwallet", owner="protocol")
revenue_bot_wallet = Wallet(address="f1revenuebot", owner="revenue-bot")

wallets = {
    "f_datacap_wallet": datacap_wallet,
    "f1sp001": sp_wallet,
    "f1revenuewallet": revenue_wallet,
    "f1vaultinfo": info_vault,
    "f1protocolwallet": protocol_wallet,
    "f1revenuebot": revenue_bot_wallet,
}

# Initialize Filecoin interface, verified list, bot, and processor
filecoin = Filecoin("f_filecoin")
verified_list = VerifiedSPList()

bot = RevenueBot(
    address="f1revenuebot",
    revenue_wallet_address="f1revenuewallet",
    info_vault_address="f1vaultinfo",
    protocol_wallet_address="f1protocolwallet",
    verified_sp_list=verified_list
)

processor = TxProcessor(wallets)

# Simulate incoming FIL payment from unverified SP
incoming = Tx(sender="f1sp001", recipient="f1revenuebot", fil_amount=100.0, signers=['f1sp001'])
print(incoming)
processor.send([incoming])
resulting_txs = bot.process_incoming_tx(incoming)

# Output and process generated transactions
for tx in resulting_txs:
    print(tx)
    processor.send([tx])

# Print final wallet balances
print("\n--- Final Wallet Balances ---")
for addr, wallet in wallets.items():
    print(f"{addr}: FIL={wallet.fil_balance}, DC={wallet.datacap_balance}")
