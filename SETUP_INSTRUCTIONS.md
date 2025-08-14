# Vehicle Net - Complete Setup Instructions

## 🚀 Quick Start (For Your Dad's Computer)

### Method 1: One-Command Setup (Recommended)

1. **Clone the repository**:
   ```bash
   git clone <your-repo-url>
   cd vehicle-net
   ```

2. **Run the complete setup script**:
   ```bash
   # On Windows with WSL/Git Bash:
   bash scripts/complete-setup.sh
   
   # Or use the Windows batch file:
   scripts/quick-setup.bat
   ```

3. **Start the web frontend**:
   ```bash
   bash scripts/run-web.sh
   ```

4. **Open your browser** to `http://localhost:3001` and go to the **Marketplace** tab. You should see 3 vehicles listed!

---

## 📋 What the Setup Script Does

The `complete-setup.sh` script automatically:

1. ✅ **Checks Anvil** - Ensures local blockchain is running
2. ✅ **Deploys Contracts** - VehicleRegistry, DataMarketplace, AccessControl
3. ✅ **Funds Wallets** - Sends ETH to vehicle wallets for registration fees
4. ✅ **Registers 3 Vehicles**:
   - Honda Civic 2023 (VIN: 1HGBH41JXMN109186)
   - Toyota Camry 2024 (VIN: 2HGBH41JXMN109187)  
   - Ford F-150 2025 (VIN: 3HGBH41JXMN109189)
5. ✅ **Generates Reports** - Creates signed condition reports for each vehicle
6. ✅ **Lists on Marketplace** - Adds data products with different prices (0.5, 0.75, 1.0 ETH)

---

## 🛠️ Prerequisites

Make sure these are installed:
- **Node.js** (for the web frontend)
- **Rust** (for the vehicle CLI)
- **Foundry** (for smart contracts) - `curl -L https://foundry.paradigm.xyz | bash`
- **Git Bash** or **WSL** (on Windows)

---

## 🔧 Manual Setup (If Script Doesn't Work)

If you need to run steps manually:

### 1. Start Blockchain
```bash
anvil
```

### 2. Deploy Contracts
```bash
cd contracts
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 forge script script/DeploySystem.s.sol --rpc-url http://localhost:8545 --broadcast
```

### 3. Fund Vehicle Wallets
```bash
# Vehicle 0
cast send 0x163c8d2c88174953a3b0ad5f60078340ed30691c --value 0.1ether --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://localhost:8545

# Vehicle 1  
cast send 0xfae38ba9cd936f66d412f4fac29fb3ba9dee8975 --value 0.1ether --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://localhost:8545

# Vehicle 5
cast send 0xa6ec3eeeb4d49443580d807d66bbc4dd9de4f1b3 --value 0.1ether --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://localhost:8545
```

### 4. Register Vehicles
```bash
# Honda Civic 2023
./scripts/run-rust.sh --index 0 --registry-address 0x5FbDB2315678afecb367f032d93F642f64180aa3 register-vehicle --vin "1HGBH41JXMN109186" --manufacturer "Honda" --model "Civic" --year 2023

# Toyota Camry 2024
./scripts/run-rust.sh --index 1 --registry-address 0x5FbDB2315678afecb367f032d93F642f64180aa3 register-vehicle --vin "2HGBH41JXMN109187" --manufacturer "Toyota" --model "Camry" --year 2024

# Ford F-150 2025
./scripts/run-rust.sh --index 5 --registry-address 0x5FbDB2315678afecb367f032d93F642f64180aa3 register-vehicle --vin "3HGBH41JXMN109189" --manufacturer "Ford" --model "F-150" --year 2025
```

### 5. List Data on Marketplace
```bash
# List Honda Civic data (0.5 ETH)
./scripts/run-rust.sh --index 0 --registry-address 0x5FbDB2315678afecb367f032d93F642f64180aa3 --marketplace-address 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 list-data --data-type "condition_report" --price 500000000000000000 --description "Honda Civic 2023 - Complete vehicle diagnostics"

# List Toyota Camry data (0.75 ETH)
./scripts/run-rust.sh --index 1 --registry-address 0x5FbDB2315678afecb367f032d93F642f64180aa3 --marketplace-address 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 list-data --data-type "condition_report" --price 750000000000000000 --description "Toyota Camry 2024 - Battery health and maintenance"

# List Ford F-150 data (1.0 ETH)
./scripts/run-rust.sh --index 5 --registry-address 0x5FbDB2315678afecb367f032d93F642f64180aa3 --marketplace-address 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 list-data --data-type "condition_report" --price 1000000000000000000 --description "Ford F-150 2025 - Comprehensive truck diagnostics"
```

---

## 📱 Expected Results

After running the setup, you should see:
- **3 registered vehicles** in the blockchain
- **3 data products** listed on the marketplace with different prices
- **Web frontend** showing all vehicles in the marketplace tab
- **Cryptographically signed** condition reports for each vehicle

---

## 🐛 Troubleshooting

**If vehicles don't appear in marketplace:**
- Check that Anvil is running on `localhost:8545`
- Verify contracts are deployed by checking the deployment logs
- Ensure all vehicle registration transactions succeeded

**If funding fails:**
- Make sure you're using the correct private key from Anvil
- Check that Anvil has sufficient ETH in the default account

**Script permission errors:**
```bash
chmod +x scripts/complete-setup.sh
chmod +x scripts/run-rust.sh
chmod +x scripts/run-web.sh
```

---

## 🎯 Final Step

After setup completes, start the web app and navigate to the Marketplace tab:
```bash
bash scripts/run-web.sh
# Open http://localhost:3001/marketplace
```

You should see all 3 vehicles with their data products listed! 🚗✨