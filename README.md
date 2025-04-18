# FIL+ Autocap
> ⚠️ **Disclaimer:** This repository is a **research prototype** designed to **simulate and experiment with auction logics** for programmable datacap allocation in the Filecoin Plus (FIL+) system.  
> It is **not intended to reflect the precise mechanics of Filecoin on-chain smart contracts**, but instead provides a simplified framework to test variations in:
>
> - Datacap allocation algorithms
> - Fee schemes
> - Burn strategies
> - Other incentive design parameters
>
> The goal is to explore and evaluate how different configurations might impact SP behavior, efficiency, and system robustness under real-world-like constraints.
> The **possible smart contract architecture** that this simulation is loosely based on can be found here:  
> 👉 [https://hackmd.io/T3cgceZaTdipgx0g32BB0Q](https://hackmd.io/T3cgceZaTdipgx0g32BB0Q)



**FIL+ Autocap** is a programmable allocator for [Filecoin Plus (FIL+)](https://docs.filecoin.io/basics/how-storage-works/filecoin-plus), designed to automatically distribute datacap to Storage Providers (SPs) based on on-chain deal revenue — without human intervention.

This approach rewards SPs who generate real paid deals and helps reduce reliance on manual verification, improving both transparency and scalability.

---

## Motivation

The current FIL+ system requires human oversight and often favors retrievable, open data — creating barriers for real-world clients with private or confidential data.

**FIL+ Autocap** aims to:

- **Eliminate human subjectivity** from the datacap allocation process  
- **Enable permissionless participation** via an economic incentive model

---

## Core Idea

The allocator runs in discrete time rounds and distributes datacap proportionally to the **reported on-chain deal revenue** by each SP.

### Key Features

- **Revenue-based rewards** — More deal revenue → more datacap  
- **Competition-based mechanism** — SPs compete for a fixed datacap pool  
- **Fee burn to resist wash trading** — A fraction of revenue is burnt to add cost to fake deals  


### Mathematical Framework

#### Datacap Allocation Mechanism
The FIL+ Autocap distributes datacap to Storage Providers (SPs) proportionally to their declared deal revenue using a competitive auction mechanism. The mathematical model that powers this allocation is designed to:
1. Reward SPs who generate real economic value through storage deals
2. Create incentives for honest revenue reporting
3. Balance short-term and long-term participation benefits

#### Core Allocation Formula
For each auction round datacap is issued to the SPs:

$$D_i(r_i) = d \cdot \frac{r_i}{\sum_j r_j}$$

Where:
- $D_i$ is the amount of datacap issued to the i-th SP
- $r_i$ is the declared deal revenue by the i-th SP
- $d$ is the total datacap issued each round
- $\sum_j r_j$ is the total declared deal revenue from all participating SPs

#### Fee Mechanism
Each SP pays a fee proportional to their declared revenue:
$$Fee_i = \gamma \cdot r_i$$

Where $\gamma$ is the fee rate parameter. A portion of this fee is burned, while the remainder is redirected to a protocol-owned wallet designated for ecosystem sustainability, ongoing development, and governance operations.

This fee structure serves as a critical economic deterrent against wash trading. Since higher declared revenues result in more datacap allocation but also incur larger fees, only SPs with genuine revenue from real user deals can sustainably participate. This creates a natural equilibrium where:

- SPs with legitimate deals can afford the fee because they receive actual payment from users
- SPs attempting to game the system through fake deals face diminishing returns as their costs increase
- The protocol captures value that is reinvested into the ecosystem

---

## Getting Started

### Requirements

- Python 3.10+
- [Poetry](https://python-poetry.org/)

### Installation

```bash
# Clone the repo
git clone https://github.com/<your-org>/filplus-autocap.git
cd filplus-autocap

# Install dependencies
poetry install
```

---

## How It Works

Once installed, you can start a test auction using the built-in CLI interface:

```bash
poetry run test-auction
```

This command starts the `MasterBot`, which triggers periodic auction rounds based on the settings in `config/setup.json`.

While the system runs, you'll interact with it via terminal commands:

```text
Enter command (register, declare, exit):
```

### Commands

#### `register`

Registers a new Storage Provider (SP) to the system.

You'll be prompted to provide:

- SP address  
- Owner address  
- Initial FIL balance

Example:
```text
Enter command (register, declare, exit): register
Enter SP address: sp1
Enter SP owner: sp1
Enter SP FIL balance: 100
```

The SP is then added to `data/verified_sp_list.json`.

#### `declare`

Used to declare deal revenue sent by a known SP to the protocol:

```text
Enter command (register, declare, exit): declare
Enter SP address: sp1
Enter revenue amount (FIL): 10
```

This will simulate a transaction from the SP to the protocol wallet, and the revenue will be considered in the next auction round.

#### `exit`

Stops the program.

---

## Auction Rounds

The `MasterBot` executes auction rounds at regular intervals. These parameters are configured in:

- `config/setup.json`:
  - `auction_duration`: Duration of a tick in seconds
  - `auction_fee`: Percentage of declared revenue to burn
  - `datacap_per_round`: Datacap to distribute each round

Auction execution is fully logged in `data/masterbot.log`. The log includes:

- Epoch ticks
- SP registration events
- Declared transactions
- Auction rounds (start, state, final allocation)

Example log excerpt:

```text
2025-04-16 15:30:26,320 - 🛰️ EVENT DETECTED: SP registration TX: <Transaction from=sp1 ...>
2025-04-16 15:30:28,558 - 🤖 MasterBot 🚀 Executing auction round number 0
2025-04-16 15:30:28,558 - 🤖 MasterBot 📦 Wallet Balances at the start: ...
2025-04-16 15:30:28,559 - 🤖 MasterBot 📊 RevenueBot Auction State:
2025-04-16 15:30:28,559 - ✅ No active contributors. Auction cleared.
```

---

## Project Structure

- `contracts/bots/` — Core logic for the allocator and its components (`master_bot.py`, `revenue_bot.py`, etc.)
- `blockchain_utils/` — Simplified blockchain abstractions (wallets, transactions, Filecoin simulation)
- `utils/` — Configuration and setup scripts

---

## Documentation

- [`docs/scheme.md`](docs/scheme.md) — Full design specification and rationale  

---

## Contributing

We welcome feedback, use cases, and pull requests.  
To propose changes or extensions, please open an issue or submit a PR.

---

## License

*To be defined*

---

## Acknowledgements

Built by [CryptoEconLab](https://github.com/CELtd) as part of ongoing research in cryptoeconomic mechanisms for Filecoin.
