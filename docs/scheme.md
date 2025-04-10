# Programmable Datacap Allocator – Functional Scheme

This document outlines the high-level operational scheme for the programmable datacap allocator. It describes the agents, contracts, and sequence of actions performed over a recurring allocation cycle.

---

## Goal

Automate and optimize the allocation of Filecoin datacap (D) to Storage Providers (SPs), based on declared deal revenues and a transparent, programmable auction mechanism.

---

## Components

### On-Chain Wallets & Contracts

- **Datacap Wallet (`Wd`)**: Holds the datacap (D) issued by Filecoin.
- **Vault (Revenue Wallet)**: Collects declared deal revenues (`rᵢ`) from SPs.
- **Datacap-Wallet-Bot**: Controls access and disbursement from the datacap wallet.
- **Revenue-Wallet-Bot**: Monitors and manages deal revenue transactions.
- **Datacap-Allocation Contract**: Logic for computing datacap distributions.
- **Datacap-Master Contract**: Coordinates auction timing and orchestrates execution.
- **Protocol Wallet**: Receives part of the fee.
- **Burn Address (`f099`)**: Receives the protocol fee share to be burned.

---

## Operational Cycle

### 1. Initialization

- Filecoin issues **D datacap every period `P`**.
- Over time `P * N`, a total of `D * N` is issued.
- Datacap is stored in **`Wd`**, managed by the **Datacap-Wallet-Bot**.

### 2. SP Registration

- Each **Storage Provider (SP)** registers:
  - Their wallet address.
  - Their eligibility to participate.
- They must interact first with the **automatic allocator**.

### 3. Auction Start

- At each time `t`, the **Datacap-Master** contract:
  - Starts a new allocation **auction**.
  - Signals the **Vault** to start accepting `rᵢ` transactions from SPs.

### 4. Deal Revenue Reception

- SPs submit their **deal revenues** `rᵢ` to the **Vault**.
- **Revenue-Wallet-Bot** checks:
  - Is the sender a registered SP?
  - Is the amount valid?

### 5. Auction End

- At time `t + T`, the **Datacap-Master**:
  - Stops the auction.
  - Locks further deal revenue submissions (if feasible on-chain).
  - Triggers the **Datacap-Allocation** contract.

### 6. Datacap Allocation Computation

- The **Datacap-Allocation** contract:
  - Reads revenues `rᵢ` from the vault.
  - Computes datacap share `dᵢ` for each SP:
    - `dᵢ = D * (rᵢ / ∑rᵢ)`
  - Calculates revenue payout:
    - SP receives: `rᵢ * (1 - x)`
    - Protocol burn: `rᵢ * x * y`
    - Protocol wallet: `rᵢ * x * (1 - y)`

### 7. Payout & Distribution

- **Revenue-Wallet-Bot** sends:
  - To SP: `rᵢ * (1 - x)`
  - To Burn Address: `rᵢ * x * y`
  - To Protocol Wallet: `rᵢ * x * (1 - y)`
- **Datacap-Wallet-Bot** sends:
  - To SP: `dᵢ`

### 8. Repeat

- The process restarts at the next period `t + P`.

---

## Parameters

| Symbol | Meaning                                        |
|--------|------------------------------------------------|
| `D`    | Datacap available in each period               |
| `P`    | Period length (e.g., weekly)                   |
| `N`    | Total number of allocation periods             |
| `x`    | Deal revenue fee fraction (0 < x < 1)          |
| `y`    | Fraction of `x` sent to burn address           |
| `rᵢ`   | Revenue declared by SP `i` in the current round|

---

