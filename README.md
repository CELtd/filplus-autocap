# FIL+ Autocap
> 🚧 **Note**: This is a work-in-progress MVP.

**FIL+ Autocap** is a programmable allocator for [Filecoin Plus (FIL+)](https://docs.filecoin.io/basics/how-storage-works/filecoin-plus), designed to automatically distribute datacap to Storage Providers (SPs) based on on-chain deal revenue — without human intervention.

This approach rewards SPs who generate real paid deals and helps reduce reliance on manual verification, improving both transparency and scalability.

---

## Motivation

The current FIL+ system requires human oversight and often favors retrievable, open data, creating barriers for real-world clients with private or confidential data.

**FIL+ Autocap** aims to:

- **Eliminate human subjectivity** from the datacap allocation process  
- **Enable permissionless participation** via an economic incentive model

---

## Core Idea

Storage Providers (SP) that made an on-chain paid deal with a client can request AutoCap for QAP reward for sealing such a deal upon payment of a fee. 
This fee is proportional to the on-chain deal payment and is known in advance to the SP, which can therefore pass it on to the client. 
Fees are partially burned to accrue value in the Filecoin economy.

Thus, the FIL+ Autocap distributes datacap to eligible Storage Providers (SPs) in rounds. 
Eligible storage providers are SPs that:
- Have a paid on-chain deal;
- Paid a fee to the Autocap proportional to the payment of the on-chain deal. 
Each round, eligible SPs receive Datacap, proportional to their share of the total fee collected by Autocap during that round. 
The larger the revenue the SP had from a on-chain deal with a client, the larger the fee that the SP issues to the allocator, and the larger the likelihood of winning more DataCap. 
Since the contribution the SP made is  never retrieved from the SP, and the DataCap received depends on the competition with the other SPs (the price of DataCap is dependent on the current state of the competition in the round) the cost of engaging with the SP needs to be backed by a real economic income from a paying client. 

### How?
1. Each SP sends a tx in FIL to the allocator wallet.  
  In each tx, the SP needs to encode in the `params` field the metadata referring to a specific on-chain deal that they will need to seal.
2. Autocap checks that the fee is paid and is valid given the on-chain deal payment.
3. Autocap listens for SP contributions for a fixed number of blocks before closing the allocation round;
4. When the round is closed, each SP is rewarded an amount of datacap proportional to their share of the total fee collected by Autocap during that round.
  E.g., SP1 paid a fee of 1 FIL, and the total fees in the round sum up to 2 FIL, then SP1 receives 50% of the DataCap prize for that round.

5. The DataCap won by each SP in each round is added to the DataCap credit of each SP.
  At the end of each round, after redistribution of the DataCap prize to each SP credit, the allocator generates the allocations.
  In particular:
  - If the `piece_size` of the deal that the SP needs to seal is larger than the SP current credit, nothing happens;
  - If the `piece_size` is smaller than or equal to the SP's current credit, the allocator creates an allocation of DataCap of size `piece_size` in the `verifreg` on-chain actor, specifying the metadata passed by the SP during the tx in step 1.

4. When the SP seals the deal, it can now claim the power from the `verifreg` actor, and the DataCap is burned.

5. A ratio of the fees accumulated during the auction round is burned. 

### Key Features

- **Revenue-based rewards** — More deal revenue → more datacap  
- **Competition-based mechanism** — SPs compete for a fixed datacap pool  
- **Fee burn to resist wash trading** — A fraction of revenue is burnt to add cost to fake deals  


### Mathematical Framework

The mathematical model that powers this allocation is designed to:
1. Reward SPs who generate real economic value through on-chain storage deals
2. Create incentives for honest revenue reporting
3. Balance short-term and long-term participation benefits
   
For each round, datacap is issued to the SPs:

$$D_i(r_i) = d \cdot \frac{r_i}{\sum_j r_j}$$

Where:
- $D_i$ is the amount of datacap issued to the i-th SP
- $r_i$ is the fee paid by the i-th SP
- $d$ is the total datacap issued each round
- $\sum_j r_j$ is the total fee collected from all participating SPs

#### Fee Mechanism
Each SP pays the allocator a fee `r_i`, which is a proxy of their real revenue `R`:
$$r_i = \gamma \cdot R_i$$

Where $\gamma$ is the fee rate parameter, a portion of the total contribution from the SPs is burned, while the remainder is redirected to a protocol-owned wallet designated for ecosystem sustainability, ongoing development, and governance operations.

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
