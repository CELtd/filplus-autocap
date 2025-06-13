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

The allocator runs in discrete time rounds.
Each round a fixed amount of datacap is at stake.
SP compete with each other to gain a part of this stake.

### How?
1. Each SP sends a tx in FIL to the allocator wallet.  
  In each tx, the SP needs to encode in the `params` field the metadata referring to a specific deal (unverified) that they will need to seal.  
2. At the end of each auction round, each SP is rewarded an amount of datacap from the total stake, proportional to the total contribution in FIL that the SP had during the auction.
  E.g. SP1 sent 1 FIL and the total FIL contribution in the auction round is 2 FIL, then the SP1 receives 50% of the DataCap at stake).

3. The DataCap won by each SP in each round is added to the DataCap credit of each SP.
  At the end of each auction round, after redistribution of the DataCap prize to each SP credit, the allocator generates the allocations.
  In particular:
  - If the `piece_size` of the deal that the SP needs to seal is larger than the SP current credit, nothing happens;
  - If the `piece_size` is smaller or equal than the SP current credit, the allocator creates an allocation of DataCap of size `piece_size` in the `verifreg` onchain actor, specifying the metadata passed by the SP during the tx in step 1.

4. When the SP seals the deal can now claim the power from the `verifreg` actor and the DataCap is burned.

5. A ratio of the FIL accumulated during the auction round is burned. 

### Key Features

- **Revenue-based rewards** — More deal revenue → more datacap  
- **Competition-based mechanism** — SPs compete for a fixed datacap pool  
- **Fee burn to resist wash trading** — A fraction of revenue is burnt to add cost to fake deals  


### Mathematical Framework

#### Datacap Allocation Mechanism
The FIL+ Autocap distributes datacap to Storage Providers (SPs) proportionally to their contribution in FIL. This contribution acts as a proxy measure of the deal revenue of the SP. Thus, in the following, we call `declared deal revenue` the payment of an SP to the allocator. The larger the revenue the SP had from a unverified deal with a client, the larger can be the payment that the SP issues to the allocator and the larger is the likelyhood of winning more DataCap. Since the contribution the SP made is  never retireved from the SP, and the DataCap received depends on the competition with the other SPs (the price of DataCap is dependent on the current state of the competition in the auction) the cost of engaging with the SP needs to be backed by a real economic income from a paying client. 

The mathematical model that powers this allocation is designed to:
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
Each SP pays a the allocator a fee `r`, which is a proxy of their real revenue `R`:
$$r_i = \gamma \cdot R_i$$

Where $\gamma$ is the fee rate parameter. A portion of the total contribution from the SPs is burned, while the remainder is redirected to a protocol-owned wallet designated for ecosystem sustainability, ongoing development, and governance operations.

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
