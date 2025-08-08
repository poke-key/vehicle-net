# Vehicle Network - Demo Setup Guide

This guide walks you through setting up a complete proof of concept demo of the Vehicle Network system, including blockchain contracts, vehicle nodes, and web interface.

## 🎯 Demo Overview

The Vehicle Network demo showcases:

- **Smart Contracts**: Vehicle registration, data marketplace, and access control
- **Vehicle Nodes**: Rust-based nodes that simulate real vehicles transmitting data
- **Web Interface**: Real-time dashboard showing vehicle data from the blockchain
- **Data Marketplace**: Vehicles selling their data with cryptographic signatures

## 📋 Prerequisites

Before starting, ensure you have the following installed:

### Required Software

- **Git** (for cloning the repository)
- **Rust** (for vehicle nodes) - [Install Rust](https://rustup.rs/)
- **Foundry** (for smart contracts) - [Install Foundry](https://book.getfoundry.sh/getting-started/installation)
- **Node.js** or **Bun** (for web interface) - [Install Node.js](https://nodejs.org/) or [Install Bun](https://bun.sh/)

### Verify Installation

```bash
# Check Rust
rustc --version
cargo --version

# Check Foundry
forge --version
anvil --version

# Check Node.js/Bun
node --version
# or
bun --version
```

## 🚀 Demo Setup Instructions

### Step 1: Clone and Setup Repository

```bash
# Clone the repository
git clone <repository-url>
cd vehicle-net

# Initialize submodules (required for contracts)
git submodule update --init --recursive
```

### Step 2: Terminal 1 - Start Blockchain (Anvil)

**Purpose**: Start a local Ethereum blockchain for testing

```bash
# Navigate to contracts directory
cd contracts

# Start Anvil blockchain
anvil
```

**Expected Output**:

```txt
🚀 Starting Anvil
Version: 2.0.0
Listening on 127.0.0.1:8545
Accounts:
[0] 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (10000 ETH)
[1] 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 (10000 ETH)
[2] 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC (10000 ETH)
...
Private Keys:
[0] 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
[1] 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
...
```

**Keep this terminal running** - this is your local blockchain!

### Step 3: Terminal 2 - Build and Deploy Smart Contracts

**Purpose**: Build and deploy the smart contracts to the local blockchain

```bash
# From project root - Build contracts first
./scripts/run-contracts.sh

# Then deploy to local blockchain (in same terminal)
./scripts/run-contracts.sh --deploy
```

**Expected Output**:

```txt
================================
Vehicle Net - Smart Contract Build and Deploy
================================

--- Initializing submodules ---
Running: git submodule update --init --recursive --force
✅ Initializing submodules completed successfully

--- Building smart contracts ---
Running: forge build
✅ Building smart contracts completed successfully

--- Build Output ---
Contracts built successfully!

Build artifacts location: contracts/out/

Built contracts:
  - AccessControl
  - DataMarketplace
  - VehicleRegistry

🎉 Smart contract build completed!

To deploy contracts to local blockchain, run:
./scripts/run-contracts.sh --deploy
Make sure anvil is running first: cd contracts && anvil

--- Deploying Smart Contracts ---
Running: forge script script/DeploySystem.s.sol --rpc-url http://localhost:8545 --broadcast --verify
✅ Deploying smart contracts completed successfully

🎉 Smart contracts deployed successfully!
ℹ️ Contracts are now available on the local blockchain
ℹ️ You can now register vehicles and start vehicle nodes
```

### Step 4: Terminal 3 - Register First Vehicle

**Purpose**: Register a vehicle on the blockchain (one-time setup per vehicle)

```bash
# From project root
./scripts/register-vehicle.sh 1 "TESLA001" "Tesla" "Model 3" 2023
```

**Expected Output**:

```txt
🚗 Registering Vehicle Node #1
VIN: TESLA001
Vehicle: 2023 Tesla Model 3
Registration Fee: 0.01 ETH

🚗 Vehicle Network - Decentralized Vehicle System
================================================
Vehicle Address: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
✅ Vehicle registered successfully!
Transaction Hash: 0x1234...
Gas Used: 123456

✅ Vehicle registered! You can now run the daemon with:
./scripts/run-vehicle-node.sh 1 TESLA001
```

### Step 5: Terminal 4 - Start First Vehicle Node

**Purpose**: Start a vehicle daemon that continuously transmits data

```bash
# From project root
./scripts/run-vehicle-node.sh 1 "TESLA001"
```

**Expected Output**:

```txt
🚗 Starting Vehicle Node #1
VIN: TESLA001
Starting Mileage: 50000
Starting Battery Health: 95%
Report Interval: 15 seconds

💡 This vehicle will continuously transmit data to the blockchain
🔗 Make sure anvil is running and contracts are deployed

🚗 Vehicle Network - Decentralized Vehicle System
================================================
🔗 Vehicle Address: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8
📡 Starting continuous data transmission...
💡 Press Ctrl+C to stop the node

📊 Report #1 - Vehicle Node Status:
   🚗 VIN: TESLA001
   🛣️  Mileage: 50003 miles
   🔋 Battery: 95%
   ⏰ Timestamp: 1703123456
   💰 Balance: 9.99 ETH
   🕒 Next report in 15 seconds

✅ Report #1 submitted to blockchain
🔏 Signature: 0x1234...
```

**Keep this terminal running** - this vehicle will continuously transmit data!

### Step 6: Terminal 5 - Register Second Vehicle (Optional)

**Purpose**: Add a second vehicle to demonstrate multiple nodes

```bash
# From project root
./scripts/register-vehicle.sh 2 "BMW002" "BMW" "i4" 2022
```

**Expected Output**:

```txt
🚗 Registering Vehicle Node #2
VIN: BMW002
Vehicle: 2022 BMW i4
Registration Fee: 0.01 ETH

🚗 Vehicle Network - Decentralized Vehicle System
================================================
Vehicle Address: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
✅ Vehicle registered successfully!
Transaction Hash: 0x5678...
Gas Used: 123456

✅ Vehicle registered! You can now run the daemon with:
./scripts/run-vehicle-node.sh 2 BMW002
```

### Step 7: Terminal 6 - Start Second Vehicle Node (Optional)

```bash
# From project root
./scripts/run-vehicle-node.sh 2 "BMW002"
```

**Expected Output**: Similar to first vehicle but with different data

### Step 8: Terminal 7 - Start Web Interface

**Purpose**: Launch the web dashboard to view real-time vehicle data

```bash
# From project root
./scripts/run-web.sh
```

**Expected Output**:

```txt
🌐 Vehicle Net - Web Application
==================================
Installing/updating dependencies...
Using Bun to update dependencies...
Starting web application...
The application will be available at: http://localhost:3001
Press Ctrl+C to stop the server

Current directory: /path/to/vehicle-net/web
Checking for package.json: -rw-r--r-- 1 user user 1234 Dec 15 10:00 package.json
Found Bun: /usr/local/bin/bun
Using Bun to run dev server...
```

**Keep this terminal running** - the web interface will be available at <http://localhost:3001>

## 🌐 Web Interface Features

Once the web interface is running, visit **<http://localhost:3001>** to see:

### Dashboard

- **Real-time vehicle data** from the blockchain
- **Live updates** every 5 seconds
- **Vehicle status** including mileage, battery health, and last report time
- **Transaction history** showing all vehicle reports

### Vehicle Management

- **Vehicle registration** forms
- **Data marketplace** listings
- **Vehicle verification** tools

### Expected Web Interface Output

```txt
🚗 Vehicle Network Dashboard

Active Vehicles:
├── TESLA001 (Tesla Model 3, 2023)
│   ├── Mileage: 50,003 miles
│   ├── Battery: 95%
│   ├── Last Report: 2 minutes ago
│   └── Status: Active
│
└── BMW002 (BMW i4, 2022)
    ├── Mileage: 60,007 miles
    ├── Battery: 92%
    ├── Last Report: 1 minute ago
    └── Status: Active

Recent Transactions:
├── Vehicle Report: TESLA001 → 50,003 miles, 95% battery
├── Vehicle Report: BMW002 → 60,007 miles, 92% battery
└── Vehicle Registration: BMW002 registered
```

## 🔧 Demo Commands Reference

### Vehicle Registration

```bash
# Register a new vehicle
./scripts/register-vehicle.sh <index> <vin> <manufacturer> <model> <year>

# Examples:
./scripts/register-vehicle.sh 1 "TESLA001" "Tesla" "Model 3" 2023
./scripts/register-vehicle.sh 2 "BMW002" "BMW" "i4" 2022
./scripts/register-vehicle.sh 3 "AUDI003" "Audi" "e-tron" 2023
```

### Vehicle Node Operation

```bash
# Start a vehicle daemon
./scripts/run-vehicle-node.sh <index> <vin> [mileage] [battery] [interval]

# Examples:
./scripts/run-vehicle-node.sh 1 "TESLA001"
./scripts/run-vehicle-node.sh 2 "BMW002" 60000 92 20
```

### Manual Rust Commands

```bash
# Test vehicle signing (offline)
cd rust
cargo run -- --index 1 sign-report --vin "TEST123" --mileage 50000 --battery-health 95

# Check vehicle balance
cargo run -- --index 1 balance

# Get vehicle information
cargo run -- --index 1 get-vehicle

# List data on marketplace
cargo run -- --index 1 list-data --data-type "GPS" --price 0.001 --description "Real-time GPS data"
```

## 🧪 Testing and Validation

### Integration Testing

Run the comprehensive integration test suite:

```bash
# From project root
./scripts/test-integration.sh
```

**Expected Output**:

```txt
🚗 Vehicle Net - Integration Test Suite
=====================================

ℹ️ This script will test:
1. Rust application compilation and execution
2. Smart contract compilation and tests
3. Contract integration between Rust and Solidity
4. Contract bindings generation and compilation

================================
Testing Rust Application
================================
✅ Vehicle 0 help command passed
✅ Vehicle 0 info command passed
✅ Vehicle 0 sign-report command passed
...

================================
Testing Smart Contracts
================================
✅ AccessControl tests passed
✅ VehicleRegistry tests passed
✅ DataMarketplace tests passed
✅ Integration tests passed

🎉 All tests completed!
```

### Individual Component Testing

#### Test Smart Contracts Only

```bash
cd contracts
forge test -vvv
```

#### Test Rust Application Only

```bash
cd rust
cargo test
cargo build --release
```

## 🐛 Troubleshooting

### Common Issues

#### 1. "Foundry not found" Error

```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

#### 2. "Rust not found" Error

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### 3. "Anvil already in use" Error

```bash
# Kill existing anvil process
pkill anvil
# Or use different port
anvil --port 8546
```

#### 4. "Contract deployment failed" Error

- Ensure anvil is running in Terminal 1
- Check that contracts built successfully: `./scripts/run-contracts.sh --build-only`
- Verify submodules are initialized
- Try deploying separately: `./scripts/run-contracts.sh --deploy`

#### 5. "Vehicle registration failed" Error

- Ensure contracts are deployed
- Check that anvil is running
- Verify vehicle index is unique

### Debug Commands

#### Check Blockchain Status

```bash
# Check if anvil is running
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545
```

#### Check Contract Addresses

```bash
# View deployed contracts (if using Foundry)
cd contracts
forge script script/DeploySystem.s.sol --rpc-url http://localhost:8545
```

#### Check Vehicle Node Status

```bash
# Test vehicle node connectivity
cd rust
cargo run -- --index 1 info
```

## 📊 Demo Metrics

### Expected Performance

- **Vehicle Reports**: Every 15-30 seconds per vehicle
- **Blockchain Confirmations**: ~1-2 seconds
- **Web Updates**: Every 5 seconds
- **Memory Usage**: ~50-100MB per vehicle node
- **CPU Usage**: ~5-10% per vehicle node

### Scaling Considerations

- **Multiple Vehicles**: Each vehicle runs in its own terminal
- **Network Load**: Minimal (local blockchain)
- **Storage**: Contract state stored on blockchain
- **Security**: Each vehicle has its own cryptographic identity

## 🎉 Demo Success Criteria

Your demo is successful when you can:

✅ **See live vehicle data** updating on the web interface  
✅ **Observe multiple vehicles** transmitting simultaneously  
✅ **View blockchain transactions** in real-time  
✅ **Verify cryptographic signatures** on vehicle reports  
✅ **Register new vehicles** through the system  
✅ **Access the data marketplace** functionality  

## 🔄 Cleanup

To stop the demo:

1. **Stop web interface**: `Ctrl+C` in Terminal 7
2. **Stop vehicle nodes**: `Ctrl+C` in Terminals 4, 6
3. **Stop anvil**: `Ctrl+C` in Terminal 1
4. **Clean test files** (optional):

   ```bash
   ./scripts/test-integration.sh --cleanup
   ```

## 📚 Next Steps

After running the demo successfully:

1. **Explore the codebase** to understand the architecture
2. **Modify vehicle parameters** to test different scenarios
3. **Add new vehicle types** with different data patterns
4. **Experiment with the data marketplace** features
5. **Deploy to testnet** for real blockchain testing

---
