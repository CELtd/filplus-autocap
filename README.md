# AutoCap

**Automated DataCap allocation for Filecoin Storage Providers based on Burn-to-Earn mechanism.**

AutoCap is a Filecoin FVM smart contract that automates the allocation of DataCap to Storage Providers (SPs) based on how much FIL they have burned through the Filecoin Pay rail.

## Overview

The contract operates on a per-round basis:

1. **Registration Phase**: SPs register by paying a fee and providing their Actor ID
2. **Calculation Phase**: Owner inputs off-chain burn verification data
3. **Distribution Phase**: DataCap is distributed proportionally to each SP's FIL burn

```
Allocation = (ParticipantBurnt × DistributableDataCap) / TotalGlobalBurnt
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         AutoCap                              │
├─────────────────────────────────────────────────────────────┤
│  State: RegistrationOpen → Calculation → Distributing       │
├─────────────────────────────────────────────────────────────┤
│  Participants                                                │
│  ├── isRegistered                                           │
│  ├── datacapActorId (destination for DataCap)               │
│  ├── filBurnt (set by oracle)                               │
│  └── hasClaimed                                             │
├─────────────────────────────────────────────────────────────┤
│  DataCap Actor (f07) ←──── FRC42 ────→ AutoCap              │
└─────────────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- Git

### Setup

```bash
# Clone the repository
git clone https://github.com/your-org/autocap-v0.git
cd autocap-v0

# Install dependencies
forge install OpenZeppelin/openzeppelin-contracts
forge install Zondax/filecoin-solidity

# Build
forge build
```

## Usage

### Build

```bash
forge build
```

### Test

```bash
forge test
```

### Test with verbosity

```bash
forge test -vvvv
```

### Format

```bash
forge fmt
```

### Gas Snapshots

```bash
forge snapshot
```

## Deployment

### Environment Setup

Create a `.env` file:

```env
PRIVATE_KEY=your_private_key
PAYMENT_CONTRACT=0x...  # Filecoin Pay contract address
REGISTRATION_FEE=100000000000000000  # 0.1 FIL in AttoFIL
```

### Deploy to Filecoin Calibration Testnet

```bash
source .env

forge script script/Deploy.s.sol:DeployAutoCap \
    --rpc-url https://api.calibration.node.glif.io/rpc/v1 \
    --broadcast
```

### Deploy Locally

```bash
# Start local node
anvil

# Deploy
forge script script/Deploy.s.sol:DeployAutoCapLocal \
    --rpc-url http://localhost:8545 \
    --broadcast
```

## Contract Interface

### For Storage Providers

```solidity
// Register for the round
function register(uint64 _datacapActorId) external payable;
```

### For Oracle (Owner)

```solidity
// Close registration phase
function closeRegistration() external;

// Set burn statistics (batched)
function setParticipantBurnStats(address[] calldata _sps, uint256[] calldata _amounts) external;

// Finalize round and enable distribution
function finalizeRound(uint256 _totalFilBurntInRound) external;

// Withdraw collected registration fees
function withdrawFees() external;
```

### For Anyone

```solidity
// Claim DataCap for beneficiaries (batched)
function claimDataCap(address[] calldata _beneficiaries) external;
```

## Events

```solidity
event Registered(address indexed sp, uint64 datacapActorId);
event BurnStatsUpdated(address indexed sp, uint256 amount);
event RoundFinalized(uint256 totalFilBurnt, uint256 distributableDataCap);
event DataCapClaimed(address indexed sp, uint256 amount);
event DataCapReceived(uint256 amount, bytes operatorData);
event RegistrationClosed();
event FeesWithdrawn(address indexed owner, uint256 amount);
```

## Security Considerations

- **Owner Trust**: The contract relies on the owner as an oracle for burn verification
- **Actor ID Validation**: Users must provide correct Actor IDs; invalid IDs result in lost DataCap
- **Gas Limits**: Batch operations (setParticipantBurnStats, claimDataCap) should be split if too many participants

## Dependencies

- [OpenZeppelin Contracts](https://github.com/OpenZeppelin/openzeppelin-contracts) - Access control
- [Filecoin Solidity](https://github.com/Zondax/filecoin-solidity) - DataCap Actor interaction

## License

MIT
