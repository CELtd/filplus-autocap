# FIL+ Autocap
> 🚧 **Note**: This is a work-in-progress MVP.

**FIL+ Autocap** is a programmable allocator for [Filecoin Plus (FIL+)](https://docs.filecoin.io/basics/how-storage-works/filecoin-plus), designed to automatically distribute datacap to Storage Providers (SPs) based on on-chain deal revenue, without human intervention.

This approach rewards SPs who generate real paid deals and helps reduce reliance on manual verification, improving both transparency and scalability.

---

## Motivation

The current FIL+ system requires human oversight and often favors retrievable, open data, creating barriers for real-world clients with private or confidential data.

**FIL+ Autocap** aims to:

- **Eliminate human subjectivity** from the datacap allocation process  
- **Enable permissionless participation** via an economic incentive model

---

## Mechanism overview
> 🚧 **Note**: Autocap is under architectural update

[**Read the new architecture here**]()


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
