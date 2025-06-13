# FIL+ Autocap
> 🚧 **Note**: This is a work-in-progress MVP.

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

### Prerequisites

- Rust (latest stable)  
  Install via [rustup.rs](https://rustup.rs)
- Lotus full node (devnet or testnet)
- `.env` file with config for RPC and keys

### Build

```bash
# Clone the repo
git clone https://github.com/<your-org>/filplus-autocap.git
cd filplus-autocap

# Build the project
cargo build
```

```bash
cargo run
```

## Contributing

We welcome feedback, use cases, and pull requests.  
To propose changes or extensions, please open an issue or submit a PR.

---

## License

*To be defined*

---

## Acknowledgements

Built by [CryptoEconLab](https://github.com/CELtd) as part of ongoing research in cryptoeconomic mechanisms for Filecoin.
