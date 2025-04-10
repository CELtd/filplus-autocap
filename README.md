# FIL+ Autocap

**FIL+ Autocap** is a programmable allocator for [Filecoin Plus (FIL+)](https://docs.filecoin.io/basics/how-storage-works/filecoin-plus), designed to automatically distribute datacap to Storage Providers (SPs) based on on-chain deal revenue — without human intervention.

This approach rewards SPs who generate real paid deals and helps reduce reliance on manual verification, improving both transparency and scalability.

---

## Motivation

The current FIL+ system requires human oversight and often favors retrievable, open data — creating barriers for real-world clients with private or confidential data.

**FIL+ Autocap** aims to:

- **Eliminate human subjectivity** from the datacap allocation process  
- **Encourage real demand** by rewarding SPs with paying clients  
- **Enable permissionless participation** via an economic incentive model

---

## Core Idea

The allocator runs in discrete time rounds and distributes datacap proportionally to the **reported on-chain deal revenue** by each SP.

### Key Features

- **Revenue-based rewards** — More deal revenue → more datacap  
- **Competition-based mechanism** — SPs compete for a fixed datacap pool  
- **Fee burn to resist wash trading** — A fraction of revenue is burnt to add cost to fake deals  

## Getting Started

### Requirements

- Python 3.10+
- Poetry

### Installation

```bash
# Clone the repo
git clone https://github.com/<your-org>/filplus-autocap.git
cd filplus-autocap

# Install dependencies
poetry install

```

---

## Documentation

- [`docs/design.md`](docs/scheme.md) — Design scheme  

---

## Contributing

We welcome feedback, use cases, and pull requests.  
To propose changes or extensions, please open an issue or submit a PR.

---

## License

---

## Acknowledgements

Built by [CryptoEconLab](https://github.com/CELtd) as part of ongoing research in cryptoeconomic mechanisms for Filecoin.
