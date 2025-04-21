# Python Package Documentation: Filplus-Autocap

## Table of Contents
- [Introduction](#introduction)
- [Package Structure](#package-structure)
- [Core Modules](#core-modules)
- [Implementation Details](#implementation-details)
- [Usage Guide](#usage-guide)
- [Development and Extension](#development-and-extension)

## Introduction

The `filplus-autocap` Python package implements a programmable datacap allocation system for the Filecoin network. This document provides a technical overview of the package structure, implementation details, and usage instructions for developers.

The package simulates a competitive auction mechanism where Storage Providers (SPs) receive datacap allocations proportional to their declared deal revenue. This creates economic alignment between value creation and resource allocation in the Filecoin ecosystem.

## Package Structure

The package follows a modular structure with clear separation of concerns:

```
filplus-autocap/
├── config/                    # Configuration files
│   └── setup.json             # Auction parameters
├── data/                      # Operational data
│   ├── masterbot.log          # Real-time logging of the results of the computation
│   └── verified_sp_list.json  # List of verified SPs
├── docs/                      # Documentation
├── src/                       # Source code
│   └── filplus_autocap/       # Main package
│       ├── __init__.py
│       ├── blockchain_utils/  # Blockchain abstractions
│       ├── contracts/         # Contract logic and bots
│       └── utils/             # Utility functions
└── pyproject.toml             # Package configuration
```

### Dependencies

The package uses [Poetry](https://python-poetry.org/) for dependency management.

## Core Modules

### Blockchain Utilities (`blockchain_utils/`)

This module provides abstractions for blockchain operations:

#### Currency Representations (`currencies.py`)

```python
class FIL(float):
    """Represents a FIL amount in the Filecoin ecosystem."""
    def __repr__(self):
        return f"{float(self):.2f} FIL"

class DAT(float):
    """Represents a Datacap amount in the Filecoin ecosystem."""
    def __repr__(self):
        return f"{float(self):.2f} DAT"
```

These classes provide type safety and semantic clarity when working with currency values.

#### Transaction Handling (`transaction.py`)

```python
class Tx:
    """Represents a transaction on the blockchain."""
    def __init__(
        self,
        sender: str,
        recipient: str,
        fil_amount: FIL,
        datacap_amount: DAT,
        signers: list[str] = None,
        message: str = "",
    ):
        self.sender = sender
        self.recipient = recipient
        self.fil_amount = fil_amount
        self.datacap_amount = datacap_amount
        self.signers = signers or []
        self.message = message
```

This class encapsulates all information needed to represent a transaction.

```python 
class TxProcessor:
    """Processes and validates a list of transactions."""
    def __init__(self, wallets: dict[str, Wallet]):
        self.wallets = wallets

    def send(self, txs: list[Tx]):
        for tx in txs:
            # Handles balance checks and asset transfers
            ...
```
This class executes transactions by updating wallet balances and optionally delegating to smart contracts like `VerifiedSPList`.

#### Wallet Implementation (`wallet.py`)

```python
class Wallet:
    """Represents a wallet in the Filecoin ecosystem."""
    def __init__(self, address: str, owner: str = "unknown"):
        self.address = address
        self.owner = owner
        self.fil_balance = FIL(0.0)
        self.datacap_balance = DAT(0.0)
```

This class serves to represent wallets.

#### Storage Provider Management (`storage_provider.py`)

```python
class StorageProvider:
    """Represents a Storage Provider in the Filecoin ecosystem."""
    def __init__(self, address: str, owner: str):
        self.address = address
        self.owner = owner
        self.fil_balance = FIL(0.0)
        self.datacap_balance = DAT(0.0)
```

This class encapsulates information about Storage Providers.

#### Filecoin Wallet (`filecoin.py`)

```python
class Filecoin(Wallet):
    """Extends Wallet to enable datacap issuance."""
    def __init__(self, address: str):
        super().__init__(address=address, owner="filecoin")
        self.datacap_balance = DAT(DATACAP_MAX_ISSUANCE)

    def issue_datacap(
        self,
        recipient_wallet: Wallet,
        amount: DAT = DAT(0),
        signers: list[str] = None
    ) -> Tx:
        return Tx(
            sender=self.address,
            recipient=recipient_wallet.address,
            datacap_amount=amount,
            fil_amount=FIL(0),
            signers=signers or [self.address],
            message=f"Mint {amount} DC to {recipient_wallet.address}"
        )
```

This class represents the Filecoin protocol’s wallet, capable of minting and issuing Datacap to other wallets via signed transactions.

---

### Contract System (`contracts/`)

This module implements the business logic for the datacap allocation process:

#### Verified SP List (`verified_sp_list.py`)

```python
class VerifiedSPList:
    """Maintains a list of verified Storage Providers."""
    def __init__(self, sp_list_file: str = None):
        self.sp_list = {}
        if sp_list_file:
            self.load_from_file(sp_list_file)
    
    def is_verified(self, address: str) -> bool:
        """Checks if a Storage Provider is verified."""
        return address in self.sp_list
    
    def add_sp(self, address: str, owner: str):
        """Adds a Storage Provider to the verified list."""
        self.sp_list[address] = {"owner": owner}
        self.save_to_file()
```

This class manages the list of verified SPs who can participate in the auction.

### Bot Framework (`contracts/bots/`)

This module contains the core logic for the auction system:

#### Base Bot Class (`bot.py`)

```python
class Bot(Wallet):
    """Base class for all bots in the system."""
    def __init__(self, address: str, owner: str = "bot"):
        super().__init__(address=address, owner=owner)
    
    def sign_tx(self, tx: Tx):
        """Signs a transaction by appending the bot's address."""
        tx.signers.append(self.address)
        return tx
```

This class extends the Wallet class with transaction signing capabilities.

#### MasterBot (`master_bot.py`)

```python
class MasterBot:
    """Coordinates auction rounds, distributes FIL and Datacap, and processes protocol/burn fees."""

    def __init__(
        address: str,
        revenue_bot: RevenueBot,
        datacap_bot: DatacapBot,
        master_fee_ratio: FIL = FIL(0.1),
        protocol_fee_ratio: FIL = FIL(0.1),
        datacap_distribution_round: DAT = DAT(1000.0),
        auction_duration: float = 10.0,
        protocol_wallet_address: str = "f1_protocol_wallet",
        burn_address: str = "f099",
        processor: TxProcessor = None
    )

    async def run_auction(time_vector: list[float]):
        """Runs auction rounds periodically based on a time vector and available Datacap."""

    async def run_auction_in_background(time_vector):
        """Asynchronous background wrapper for `run_auction`."""

    def execute_auction_round(self) -> None:
    """
    Executes a single auction round by distributing FIL refunds and Datacap rewards to verified SPs
    proportional to their contributions. After rewards are distributed, the remaining FIL is split
    between a protocol fee and a burn fee to maintain economic sustainability.

    This function performs the following steps:
      1. Drains the FIL contributions (auction state) from the RevenueBot.
      2. Calculates each SP's relative contribution (c_i) to determine:
         - FIL refund amount (minus the master fee).
         - Datacap issuance amount.
      3. Issues two transactions per SP:
         - One refunding FIL.
         - One issuing Datacap.
      4. Computes the leftover balance (i.e., total fees collected via master fee).
      5. Splits the leftover into:
         - Burned FIL (sent to a burn address).
         - Protocol fee (sent to the protocol treasury).
      6. Emits and sends all transactions.

    This function encodes the incentive logic of the system: contributors are rewarded with
    both refunds and Datacap, while a portion of FIL is retained by the protocol and burned
    to align long-term economic and governance incentives.
    """

```

This class governs the full auction lifecycle, coordinating with `RevenueBot` and `DatacapBot` to fairly allocate resources and maintain protocol economics.

#### RevenueBot (`revenue_bot.py`)

```python
class RevenueBot(Bot):
    """
    Tracks FIL contributions from verified SPs and redirects unverified contributions to the protocol wallet.
    """

    def __init__(
        self,
        address: str,
        protocol_wallet_address: str,
        verified_sp_list: VerifiedSPList,
    ):
        super().__init__(address=address, owner=["revenue_bot", "master_bot"])
        self.protocol_wallet_address = protocol_wallet_address
        self.verified_sp_list = verified_sp_list
        self.current_auction: dict[str, float] = {}

    def process_incoming_tx(self, tx: Tx) -> list[Tx]:
        """
        Records FIL from verified SPs or redirects it if unverified.
        """
        sender = tx.sender
        fil_amount = tx.fil_amount
        outgoing_txs = []

        if not self.verified_sp_list.is_verified(sender):
            outgoing_txs.append(
                Tx(
                    sender=self.address,
                    recipient=self.protocol_wallet_address,
                    fil_amount=FIL(fil_amount),
                    datacap_amount=DAT(0.0),
                    signers=[self.address],
                    message=f"Redirected revenue from unverified SP {sender}",
                )
            )
        else:
            self.current_auction[sender] = self.current_auction.get(sender, FIL(0.0)) + FIL(fil_amount)

        return outgoing_txs

    def drain_auction(self) -> dict[str, float]:
        """
        Returns and clears current auction contributions.
        """
        drained = self.current_auction.copy()
        self.current_auction.clear()
        return drained

    def create_fil_tx(self, recipient_address: str, fil_amount: FIL, message : str = f"FIL tx"):
        """
        Creates a transaction for transferring FIL from the RevenueBot's associated wallet.
        
        Args:
            recipient_address (str): The address to send FIL to.
            fil_amount (FIL): The amount of FIL to send.
        
        Returns:
            Tx: The generated transaction that is signed by the RevenueBot.
        """
        # Create a new transaction where:
        # - The sender is the revenuebot_wallet's address
        # - The recipient is the specified recipient address
        # - The FIL amount is specified, and no DAT is involved in this transaction
        tx = Tx(
            sender=self.address,  # Sender is the datacap_wallet's address
            recipient=recipient_address,  # Recipient is the address passed in
            datacap_amount=DAT(0), 
            fil_amount=FIL(fil_amount), 
            signers=[],  # The list of signers is initially empty
            message=message
        )
        
        # Return the signed transaction
        return self.sign_tx(tx)
```

The RevenueBot tracks FIL contributions from verified SPs.

#### DatacapBot (`datacap_bot.py`)

```python
class DatacapBot(Bot):
    """Manages datacap allocation."""
    def __init__(self, address: str, datacap_wallet: Wallet):
        super().__init__(address)
        self.datacap_wallet = datacap_wallet
    
    def get_datacap_balance(self):
        """Gets the current datacap balance."""
        return self.datacap_wallet.datacap_balance
    
    def create_datacap_tx(self, recipient_address: str, datacap_amount: DAT):
        """Creates a datacap transfer transaction."""
        tx = Tx(
            sender=self.datacap_wallet.address,
            recipient=recipient_address,
            datacap_amount=DAT(datacap_amount),
            fil_amount=FIL(0.0),
            signers=[],
        )
        
        self.sign_tx(tx)
        return tx
```

The DatacapBot handles datacap distribution.

### Utility Components (`utils/`)

This module provides support functions:

#### Logging System (`logger.py`)

Configures the logging system using the standard Python logging module.

#### Configuration Management (`setup.py`)

Loads and manages configuration parameters from JSON files.

#### Console Interface (`console.py`)

Provides a command-line interface for interacting with the system.

## Implementation Details

### Auction Cycle

The auction cycle is implemented as follows:

1. **Initialization**: The system is initialized with configuration parameters.
2. **SP Registration**: SPs register with the system.
3. **Revenue Declaration**: SPs declare their deal revenue.
4. **Auction Execution**: The system executes auction rounds at regular intervals.
5. **Cycle Repetition**: The process repeats for subsequent rounds.

The core auction logic is implemented in the `execute_auction_round` method of the MasterBot:

```python
def execute_auction_round(self):
    """Executes a single auction round."""
    # Get the current auction state
    auction_state = self.revenue_bot.drain_auction()
    
    # Calculate datacap allocations
    datacap_allocations = self.calculate_datacap_allocations(auction_state)
    
    # Distribute datacap to SPs
    self.distribute_datacap(datacap_allocations)
    
    # Distribute rewards (fee mechanism)
    self.distribute_rewards(auction_state)
```

### Mathematical Model

The datacap allocation formula is:

$$D_i(r_i) = d \cdot \frac{r_i}{\sum_j r_j}$$

Where:
- $D_i$ is the datacap issued to SP $i$
- $r_i$ is the declared revenue by SP $i$
- $d$ is the total datacap per round
- $\sum_j r_j$ is the total declared revenue

This is implemented in the `calculate_datacap_allocations` method:

```python
def calculate_datacap_allocations(self, auction_state: dict[str, float]) -> dict[str, float]:
    """Calculates datacap allocations based on the auction state."""
    total_revenue = sum(auction_state.values())
    
    if total_revenue == 0:
        return {}
    
    allocations = {}
    for sp, revenue in auction_state.items():
        allocation = self.datacap_distribution_round * (revenue / total_revenue)
        allocations[sp] = allocation
    
    return allocations
```

### Fee Mechanism

The fee mechanism is implemented in the `distribute_rewards` method:

```python
def distribute_rewards(self, auction_state: dict[str, float]):
    """Distributes rewards based on the auction state."""
    reward_txs = []
    
    total_balance = self.revenue_bot.fil_balance
    
    burn_amount = total_balance * self.burn_ratio
    leftover_balance = total_balance - burn_amount
    protocol_fee_amount = leftover_balance * self.protocol_fee_ratio
    
    # Create burn transaction
    reward_txs.append(
        Tx(
            sender=self.revenue_bot.address,
            recipient=self.burn_address,
            fil_amount=FIL(burn_amount),
            datacap_amount=DAT(0.0),
            signers=[self.revenue_bot.address, self.address],
            message="Burned FIL",
        )
    )
    
    # Create protocol fee transaction
    reward_txs.append(
        Tx(
            sender=self.revenue_bot.address,
            recipient=self.protocol_wallet_address,
            fil_amount=FIL(protocol_fee_amount),
            datacap_amount=DAT(0.0),
            signers=[self.revenue_bot.address, self.address],
            message="Protocol fee",
        )
    )
    
    # Execute transactions
    for tx in reward_txs:
        self.processor.send([tx])
```

## Usage Guide

### Installation

```bash
# Clone the repository
git clone https://github.com/<your-org>/filplus-autocap.git
cd filplus-autocap

# Install dependencies using Poetry
poetry install
```

### Running the Test Auction

The package includes a test auction system that can be run with:

```bash
poetry run test-auction
```

This starts the MasterBot, which triggers periodic auction rounds based on the settings in `config/setup.json`.

### Command Interface

The test auction provides a command-line interface:

```
Enter command (register, declare, exit):
```

#### Register a Storage Provider

```
Enter command (register, declare, exit): register
Enter SP address: sp1
Enter SP owner: sp1
Enter SP FIL balance: 100
```

This adds the SP to `data/verified_sp_list.json`.

#### Declare Deal Revenue

```
Enter command (register, declare, exit): declare
Enter SP address: sp1
Enter revenue amount (FIL): 10
```

This simulates a transaction from the SP to the protocol wallet.

#### Exit the Program

```
Enter command (register, declare, exit): exit
```

### Configuration

The system behavior is configured in `config/setup.json`:

```json
{
  "auction_duration": 10,
  "auction_fee": 0.1,
  "datacap_per_round": 1000
}
```

- `auction_duration`: Duration of a tick in epochs
- `auction_fee`: Percentage of declared revenue to burn
- `datacap_per_round`: Datacap to distribute each round

### Logging

Auction execution is logged in `data/masterbot.log`, including:
- Epoch ticks
- SP registration events
- Declared transactions
- Auction rounds (start, state, final allocation)

## Development and Extension

### Adding a New Bot

Create a new bot by extending the `Bot` class:

```python
from filplus_autocap.contracts.bots.bot import Bot

class CustomBot(Bot):
    """A custom bot with additional functionality."""
    def __init__(self, address: str):
        super().__init__(address)
        # Additional initialization
    
    def custom_method(self):
        """Custom functionality."""
        # Implementation
```

### Modifying Auction Parameters

Modify the auction parameters in `config/setup.json` to experiment with different behaviors.

### Testing Framework

The test environment in `test_interactive_auction.py` provides a framework for testing new features and modifications.

### Extending the Command Interface

Add new commands to the command interface in `console.py`:

```python
def process_command(command: str):
    if command == "register":
        # Registration logic
    elif command == "declare":
        # Declaration logic
    elif command == "new_command":
        # New command logic
    elif command == "exit":
        return False
    return True
```

## Conclusion

The `filplus-autocap` Python package provides a flexible and extensible implementation of a programmable datacap allocation system for the Filecoin network. Its modular design, clear separation of concerns, and comprehensive documentation make it an excellent starting point for developing and testing new allocation mechanisms.

For more information, refer to the README.md and other documentation in the repository.
