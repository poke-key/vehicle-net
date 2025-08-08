# CLAUDE.md

## Purpose

This project simulates a decentralized vehicle network where each **vehicle is a node** that can:

- Derive a unique Ethereum wallet from a shared HD mnemonic
- Sign vehicle condition reports (e.g., mileage, battery health, timestamp)
- Submit signed reports to a smart contract-based marketplace
- Enable buyers to verify the authenticity of a vehicle via cryptographic signatures

This forms the foundation of a **trustless vehicle marketplace**, where verified data is displayed on a website and payments are handled via on-chain tools like Porto.

---

## Project Overview

Vehicle Net is a decentralized vehicle condition reporting system with three main components:

- **Rust application** (`rust/`) - Vehicle condition reporting and cryptographic signing
- **Smart contracts** (`contracts/`) - Solidity contracts for vehicle registry and data marketplace using Foundry
- **Web application** (`web/`) - Next.js frontend with Web3 integration using Wagmi and Viem

## Common Commands

### Rust Application

```bash
# Build and run from root (recommended)
./run-rust.sh
./run-rust.sh --index 5  # Run with specific vehicle index

# From rust directory
cd rust && cargo run -- --index 0
```

### Smart Contracts (Foundry)

```bash
cd contracts
forge build      # Build contracts
forge test        # Run tests
forge fmt         # Format code
forge snapshot    # Gas snapshots
anvil            # Start local node
```

### Web Application

```bash
cd web
npm install      # or bun install
npm run dev      # Start dev server on port 3001
npm run build    # Build for production
npm run lint     # Lint code
```

## Architecture

### Smart Contract System

- **VehicleRegistry.sol** - Core contract for vehicle registration with VIN mapping and metadata
- **DataMarketplace.sol** - Handles data access purchases and vehicle data listing
- **AccessControl.sol** - Manages permissions and authorization
- Uses OpenZeppelin for security patterns (Ownable, ReentrancyGuard)

### Rust Component

- HD wallet derivation for different vehicles using master mnemonic
- Vehicle condition report generation (VIN, mileage, battery health, timestamp)
- Cryptographic signing of reports for blockchain compatibility
- Uses ethers-rs for Ethereum integration

### Web Frontend

- Next.js 15 with TypeScript and TailwindCSS
- Web3 integration via Wagmi v2 and Viem v2
- React Query for state management
- shadcn/ui components
- Runs on port 3001

### Key Integration Points

- Rust app generates signed vehicle reports
- Smart contracts store vehicle registry and handle data marketplace
- Web app provides UI for vehicle registration and data access purchases
- useVehicleContract hook abstracts contract interactions

## Development Notes

### Contract Development

- All contracts use Solidity ^0.8.19
- Gas optimization enabled with 200 runs
- Environment variables required for RPC endpoints and API keys (.env)

### Web Development  

- TypeScript strict mode enabled
- Uses Wagmi hooks for blockchain interactions
- Mock contract data available in lib/mock-data for development

### Testing

- there is no testing this is for a hackathon lol. just make sure there are no errors when building / compiling.
