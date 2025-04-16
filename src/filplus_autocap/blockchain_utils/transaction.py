from typing import List, Optional, TYPE_CHECKING
from filplus_autocap.blockchain_utils.wallet import Wallet
from filplus_autocap.blockchain_utils.currencies import FIL, DAT
from filplus_autocap.utils.constants import FILECOIN_ADDRESS

if TYPE_CHECKING:
    from filplus_autocap.contracts.verified_sp_list import VerifiedSPList


class Tx:
    def __init__(
        self,
        sender: str,
        recipient: str,
        datacap_amount: DAT = DAT(0),
        fil_amount: FIL = FIL(0),
        signers: Optional[List[str]] = None,
        message: Optional[str] = None
    ):
        self.sender = sender
        self.recipient = recipient
        self.datacap_amount = DAT(datacap_amount)
        self.fil_amount = FIL(fil_amount)
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

    def send(self, txs: List[Tx]):
        for tx in txs:
            sender_wallet = self.wallets.get(tx.sender)
            recipient_wallet = self.wallets.get(tx.recipient)

            if recipient_wallet is None:
                raise ValueError(f"Missing recipient wallet in tx: {tx}")

            if recipient_wallet.__class__.__name__ == "VerifiedSPList":
                recipient_wallet.process_tx(tx)
                continue

            if sender_wallet is None:
                raise ValueError(f"Missing sender wallet in tx: {tx}")

            if tx.sender != FILECOIN_ADDRESS and sender_wallet.fil_balance < tx.fil_amount:
                raise ValueError(
                    f"Insufficient FIL {sender_wallet.fil_balance} in sender wallet: {tx.sender}"
                )

            if sender_wallet.datacap_balance < tx.datacap_amount:
                raise ValueError(
                    f"Insufficient Datacap {sender_wallet.datacap_balance} in sender wallet: {tx.sender}"
                )

            # Process FIL transfer
            sender_wallet.fil_balance -= tx.fil_amount
            recipient_wallet.fil_balance += tx.fil_amount

            # Process Datacap transfer
            sender_wallet.datacap_balance -= tx.datacap_amount
            recipient_wallet.datacap_balance += tx.datacap_amount
