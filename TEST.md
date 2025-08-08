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

### 📋 Terminal Overview

This demo uses a multi-terminal approach to simulate a real vehicle network:

- **Terminal 1**: Smart Contract Operations (blockchain building & deployment)
- **Terminal 2**: Web Application Development (frontend interface)
- **Terminal 3+**: Vehicle Nodes (individual vehicles on the blockchain)

Each terminal represents a different component of the vehicle network system.

### Step 1: Clone and Setup Repository

```bash
# Clone the repository
git clone <repository-url>
cd vehicle-net

# Initialize submodules (required for contracts)
git submodule update --init --recursive
```

### Step 2: Terminal 1 - Smart Contract Operations

**Purpose**: Build and deploy smart contracts to the local blockchain

```bash
# From project root - Build and deploy contracts
./scripts/terminal-contracts.sh build-and-deploy
```

**Expected Output**:

```txt
================================
Terminal 1: Smart Contract Operations
================================
ℹ️ This terminal handles blockchain contract building and deployment

--- Initializing submodules ---
Running: git submodule update --init --recursive --force
✅ Initializing submodules completed successfully

--- Building smart contracts ---
Running: forge build
✅ Building smart contracts completed successfully

🎉 Smart contract build completed!
ℹ️ Build artifacts location: contracts/out/
ℹ️ Built contracts: AccessControl, DataMarketplace, VehicleRegistry

--- Deploying Smart Contracts ---
Running: forge script script/DeploySystem.s.sol --rpc-url http://localhost:8545 --broadcast --verify
✅ Deploying smart contracts completed successfully

🎉 Smart contracts deployed successfully!
ℹ️ Contracts are now available on the local blockchain
ℹ️ You can now register vehicles and start vehicle nodes
```

**Keep this terminal available** - you can use it to manage contracts and blockchain operations!

### Step 3: Terminal 2 - Web Application Development

**Purpose**: Build and run the web interface

```bash
# From project root - Start web development server
./scripts/terminal-web.sh dev
```

**Expected Output**:

```txt
================================
Terminal 2: Web Application Development
================================
ℹ️ This terminal handles the web interface building and development

--- Installing Dependencies ---
Using Bun to install/update dependencies...
✅ Dependencies installed successfully

--- Starting Development Server ---
Starting web application...
The application will be available at: http://localhost:3001
Press Ctrl+C to stop the server

Current directory: /path/to/vehicle-net/web
Checking for package.json: -rw-r--r-- 1 user user 1234 Dec 15 10:00 package.json
Found Bun: /usr/local/bin/bun
Using Bun to run dev server...
```

**Keep this terminal running** - the web interface will be available at <http://localhost:3001>

### Step 4: Terminal 3 - Register First Vehicle

**Purpose**: Register a vehicle on the blockchain (one-time setup per vehicle)

```bash
# From project root
./scripts/terminal-vehicle.sh 1 register "TESLA001" "Tesla" "Model 3" 2023
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
./scripts/terminal-vehicle.sh 1 daemon "TESLA001"
```

**Expected Output**:

```txt
================================
Terminal 3+: Vehicle Node Operations
================================
ℹ️ This terminal handles vehicle registration and data transmission

--- Starting Vehicle Daemon ---
Vehicle Index: 1
VIN: TESLA001
Starting Mileage: 50000
Starting Battery Health: 95%
Report Interval: 15 seconds

✅ Anvil is running on localhost:8545
ℹ️ This vehicle will continuously transmit data to the blockchain
ℹ️ Press Ctrl+C to stop the daemon

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
./scripts/terminal-vehicle.sh 2 register "BMW002" "BMW" "i4" 2022
```

**Expected Output**:

```txt
================================
Terminal 3+: Vehicle Node Operations
================================
ℹ️ This terminal handles vehicle registration and data transmission

--- Registering Vehicle ---
Vehicle Index: 2
VIN: BMW002
Vehicle: 2022 BMW i4
Registration Fee: 0.01 ETH

🚗 Vehicle Network - Decentralized Vehicle System
================================================
Vehicle Address: 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
✅ Vehicle registered successfully!
Transaction Hash: 0x5678...
Gas Used: 123456

✅ Vehicle registered successfully!
ℹ️ You can now start the vehicle daemon with:
ℹ️ ./scripts/terminal-vehicle.sh 2 daemon BMW002
```

### Step 7: Terminal 6 - Start Second Vehicle Node (Optional)

```bash
# From project root
./scripts/terminal-vehicle.sh 2 daemon "BMW002"
```

**Expected Output**: Similar to first vehicle but with different data

**Expected Output**: Similar to first vehicle but with different data

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

### Terminal 1: Smart Contract Operations

```bash
# Build and deploy contracts
./scripts/terminal-contracts.sh build-and-deploy

# Check blockchain status
./scripts/terminal-contracts.sh status

# Start anvil blockchain
./scripts/terminal-contracts.sh start-anvil

# Stop anvil blockchain
./scripts/terminal-contracts.sh stop-anvil
```

### Terminal 2: Web Application Development

```bash
# Start development server
./scripts/terminal-web.sh dev

# Build for production
./scripts/terminal-web.sh build

# Check web status
./scripts/terminal-web.sh status

# Install dependencies
./scripts/terminal-web.sh install
```

### Terminal 3+: Vehicle Operations

```bash
# Register a new vehicle
./scripts/terminal-vehicle.sh <index> register <vin> [manufacturer] [model] [year] [fee]

# Start vehicle daemon
./scripts/terminal-vehicle.sh <index> daemon <vin> [mileage] [battery] [interval]

# Get vehicle information
./scripts/terminal-vehicle.sh <index> info

# Check vehicle balance
./scripts/terminal-vehicle.sh <index> balance

# Sign a report (offline)
./scripts/terminal-vehicle.sh <index> sign <vin> [mileage] [battery]

# List data on marketplace
./scripts/terminal-vehicle.sh <index> list <data_type> [price] [description]

# Examples:
./scripts/terminal-vehicle.sh 1 register "TESLA001" "Tesla" "Model 3" 2023 0.01
./scripts/terminal-vehicle.sh 1 daemon "TESLA001" 50000 95 15
./scripts/terminal-vehicle.sh 1 info
./scripts/terminal-vehicle.sh 1 balance
./scripts/terminal-vehicle.sh 1 sign "TESLA001" 50003 95
./scripts/terminal-vehicle.sh 1 list "GPS" 0.001 "Real-time GPS data"
```

## 🧪 Testing and Validation

### Integration Testing

#### Step 1: Build Smart Contracts

Before running integration tests, ensure all smart contracts are built:

```bash
# From project root - Build contracts first
./scripts/run-contracts.sh
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
```

#### Step 2: Run Integration Test Suite

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

- Ensure anvil is running: `./scripts/terminal-contracts.sh status`
- Check that contracts built successfully: `./scripts/terminal-contracts.sh build`
- Verify submodules are initialized
- Try deploying separately: `./scripts/terminal-contracts.sh deploy`

#### 5. "Vehicle registration failed" Error

- Ensure contracts are deployed: `./scripts/terminal-contracts.sh status`
- Check that anvil is running: `./scripts/terminal-contracts.sh status`
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
# View deployed contracts
./scripts/terminal-contracts.sh status
```

#### Check Vehicle Node Status

```bash
# Test vehicle node connectivity
./scripts/terminal-vehicle.sh 1 info
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

1. **Stop web interface**: `Ctrl+C` in Terminal 2
2. **Stop vehicle nodes**: `Ctrl+C` in Terminals 4, 6
3. **Stop anvil**: `./scripts/terminal-contracts.sh stop-anvil`
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
