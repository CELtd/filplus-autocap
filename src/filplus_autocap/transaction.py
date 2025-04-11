from typing import List, Optional
from filplus_autocap.wallet import Wallet
from filplus_autocap.constants import GAS_PRICE, FILECOIN_ADDRESS
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from filplus_autocap.verified_sp_list import VerifiedSPList

class Tx:
    def __init__(
        self,
        sender: str,
        recipient: str,
        datacap_amount: float = 0.0,
        fil_amount: float = 0.0,
        signers: Optional[List[str]] = None,
        message: Optional[str] = None
    ):
        self.sender = sender
        self.recipient = recipient
        self.datacap_amount = datacap_amount
        self.fil_amount = fil_amount
        self.signers = signers or []
        self.message = message

    def __repr__(self):
        return (
            f"<Transaction from={self.sender} to={self.recipient} "
            f"datacap={self.datacap_amount} FIL={self.fil_amount} "
            f"signers={self.signers} message={self.message!r}>"
        )


class TxProcessor:
    def __init__(self, wallets: dict[str, Wallet]):
        self.wallets = wallets

    def send(self, txs: list[Tx]):
        for tx in txs:
            sender_wallet = self.wallets.get(tx.sender)
            recipient_wallet = self.wallets.get(tx.recipient)
            # If tx to verified list, register the account
            if recipient_wallet.__class__.__name__ == "VerifiedSPList":
                recipient_wallet.process_tx(tx)
            else:
                if sender_wallet is None or recipient_wallet is None:
                    raise ValueError(f"Missing wallet for sender or recipient in tx: {tx}")

                if sender_wallet.fil_balance < tx.fil_amount + GAS_PRICE and tx.sender != FILECOIN_ADDRESS:
                    raise ValueError(f"Insufficient FIL in sender wallet: {tx.sender}")

                if sender_wallet.datacap_balance < tx.datacap_amount:
                    raise ValueError(f"Insufficient Datacap in sender wallet: {tx.sender}")

                # Process FIL transfer
                if tx.sender == FILECOIN_ADDRESS:
                    sender_wallet.fil_balance -= (tx.fil_amount)
                else:
                    sender_wallet.fil_balance -= (tx.fil_amount + GAS_PRICE)
                recipient_wallet.fil_balance += tx.fil_amount

                # Process Datacap transfer
                sender_wallet.datacap_balance -= tx.datacap_amount
                recipient_wallet.datacap_balance += tx.datacap_amount