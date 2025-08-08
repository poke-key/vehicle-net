# Vehicle Network Testing Guide

This guide walks you through testing the complete Vehicle Network system from scratch.

## Prerequisites

You should have these installed and working:

- Node.js (v18+) and npm/bun
- Rust (latest stable)
- Foundry (forge, anvil, cast)

Nothing needs to be built yet - we'll do that as part of the test.

## Step-by-Step Testing

### 1. Deploy Contracts and Start Blockchain

First, deploy the smart contracts and start a local Anvil blockchain:

```bash
source scripts/deploy.sh
```

**What this does:**

- Builds all Solidity contracts with Foundry
- Starts Anvil on localhost:8545 with a known mnemonic
- Deploys VehicleRegistry, DataMarketplace, and AccessControl contracts
- Saves contract addresses to `scripts/config.sh`

**Expected output:**

```txt
🔧 Starting Vehicle Network Deployment...
🚀 Starting Anvil...
⏳ Waiting for Anvil to start...
✅ Anvil started successfully
🔨 Building contracts...
📋 Deploying contracts...
✅ Deployment Complete!
════════════════════════════════════════════════
🌐 RPC URL: http://127.0.0.1:8545
🔑 Private Key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
📋 VehicleRegistry: 0x5FbDB2315678afecb367f032d93F642f64180aa3
🏪 DataMarketplace: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
🔐 AccessControl: 0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0
════════════════════════════════════════════════
```

**Troubleshooting:**

- If Anvil fails to start, make sure port 8545 is free
- If deployment fails, check that Foundry is properly installed

### 2. Start the Frontend (New Terminal)

Open a **new terminal** and start the Next.js frontend:

```bash
./scripts/dev_frontend.sh
```

**What this does:**

- Loads contract addresses from the config file
- Installs frontend dependencies (if needed)
- Sets environment variables for the contract addresses
- Starts Next.js dev server on port 3001

**Expected output:**

```
🌐 Starting Vehicle Network Frontend...
📋 Using configuration:
RPC URL: http://127.0.0.1:8545
Registry: 0x5FbDB2315678afecb367f032d93F642f64180aa3
Marketplace: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
📦 Installing frontend dependencies...
🚀 Starting Next.js development server...
Frontend will be available at: http://localhost:3001
```

**Troubleshooting:**

- If config not found, make sure step 1 completed successfully
- If port 3001 is busy, the script will fail - kill any existing Next.js processes

### 3. Register Some Vehicles (New Terminal)

Open **another new terminal** and register a few test vehicles:

```bash
# Register first vehicle
./scripts/run_vehicle.sh VIN123ABC 12000 85

# Register second vehicle  
./scripts/run_vehicle.sh VIN456DEF 25000 78

# Register third vehicle
./scripts/run_vehicle.sh VIN789GHI 5000 95
```

**What each command does:**

- Builds the Rust vehicle client (first run only)
- Generates a unique wallet for the vehicle based on VIN hash
- Registers the vehicle on the blockchain with the given specs
- Submits a condition report with mileage and battery health
- Shows vehicle info and balance

**Expected output for each vehicle:**

```
🚗 Starting Vehicle: VIN123ABC
📋 Using configuration:
RPC URL: http://127.0.0.1:8545
Registry: 0x5FbDB2315678afecb367f032d93F642f64180aa3
Vehicle: VIN123ABC (12000mi, 85% battery)
🔨 Building Rust vehicle client...
🔧 Vehicle Index: 42
📝 Registering vehicle...
✅ Vehicle registered successfully
📊 Submitting condition report...
✅ Condition report submitted
ℹ️  Vehicle Information:
🚗 Vehicle Information:
ID: 1
VIN: VIN123ABC
Wallet: 0x742d35Cc6634C0532925a3b8D403B0b3B4bf66E9
Manufacturer: AutoGen
Model: Model-ABC
Year: 2023
Active: true
Registration Time: 1704067200
Owner: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

🎉 Vehicle VIN123ABC is now active on the blockchain!
```

**Troubleshooting:**

- If Rust build fails, make sure Rust toolchain is installed
- If registration fails, check that Anvil is still running
- Each vehicle gets a unique index based on VIN hash, so same VIN = same wallet

### 4. Test the Frontend

1. **Open browser** to <http://localhost:3001>
2. **Landing page** should load with the Vehicle Network homepage
3. **Click "Explore Marketplace"** to go to the vehicle listings
4. **Authentication required** - you'll see a Porto login screen
5. **Click "Sign in with Porto"** to authenticate
6. **View vehicles** - you should see your registered vehicles with:
   - Real data from the blockchain (VIN, manufacturer, model, year)
   - Vehicle wallet addresses
   - Registration timestamps
   - Available data types (GPS, Diagnostics, Fuel)
   - IPFS hash for metadata

### 5. Verify Data is from Blockchain

To confirm the frontend is reading real blockchain data:

1. **Register another vehicle** in terminal:

   ```bash
   ./scripts/run_vehicle.sh VIN999TEST 50000 65
   ```

2. **Refresh the frontend** - the new vehicle should appear immediately

3. **Check vehicle details** by clicking "View Details" on any vehicle

### 6. Test Vehicle Updates

You can submit new condition reports for existing vehicles:

```bash
# Update the first vehicle with new mileage/battery
./scripts/run_vehicle.sh VIN123ABC 13000 83
```

The vehicle data should update in the frontend (refresh page to see changes).

## Expected Test Results

If everything works correctly, you should see:

✅ **Blockchain**: Anvil running with deployed contracts  
✅ **Frontend**: Next.js app on port 3001 with Porto authentication  
✅ **Vehicles**: 3+ vehicles registered and visible in the frontend  
✅ **Data Flow**: Real blockchain data displayed in the UI  
✅ **Updates**: New vehicles appear when registered  

## Troubleshooting Common Issues

### "Configuration not found" Error

- Make sure `scripts/deploy.sh` completed successfully
- Check that `scripts/config.sh` exists and has contract addresses

### Frontend Won't Connect to Blockchain

- Verify Anvil is still running (check first terminal)
- Make sure contract addresses are set correctly
- Check browser console for errors

### Vehicle Registration Fails

- Confirm the vehicle hasn't already been registered with that VIN
- Check that you have sufficient ETH balance (Anvil gives test accounts 10,000 ETH)
- Verify contract addresses are correct in the config

### Porto Authentication Issues

- Make sure you're testing in a browser that supports MetaMask/wallet connections
- Try refreshing the page if auth seems stuck

## Cleanup

To stop everything:

1. **Stop frontend**: Ctrl+C in the frontend terminal
2. **Stop Anvil**: Ctrl+C in the deploy terminal  
3. **Clean up**: `rm scripts/config.sh` to reset for next test

## Next Steps

Once this basic test works, you can:

- Register more vehicles with different specs
- Test the data marketplace features
- Experiment with different VINs and vehicle parameters
- Check vehicle balances and transaction history
- Test Porto payment flows (if implemented)
