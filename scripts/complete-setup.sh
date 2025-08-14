#!/bin/bash

# Complete Vehicle Net Setup Script
# This script sets up the entire system with 3 pre-registered vehicles

set -e

echo "🚗 Vehicle Net Complete Setup Script"
echo "====================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_step() {
    echo -e "${BLUE}🔧 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Function to check and install dependencies
check_dependencies() {
    print_step "Checking dependencies..."
    
    # Check if forge is installed
    if ! command -v forge &> /dev/null; then
        print_warning "Foundry not found. Installing Foundry..."
        curl -L https://foundry.paradigm.xyz | bash
        source ~/.bashrc || source ~/.bash_profile || true
        foundryup
        if ! command -v forge &> /dev/null; then
            print_error "Failed to install Foundry. Please install manually: https://book.getfoundry.sh/getting-started/installation"
            exit 1
        fi
        print_success "Foundry installed successfully"
    else
        print_success "Foundry is available"
    fi
    
    # Check if jq is installed
    if ! command -v jq &> /dev/null && ! command -v jq.exe &> /dev/null; then
        print_warning "jq not found. Installing jq..."
        mkdir -p ~/bin
        curl -L https://github.com/stedolan/jq/releases/latest/download/jq-win64.exe -o ~/bin/jq.exe
        chmod +x ~/bin/jq.exe
        export PATH="$HOME/bin:$PATH"
        if ! command -v jq.exe &> /dev/null; then
            print_error "Failed to install jq. Please install manually"
            exit 1
        fi
        print_success "jq installed successfully"
    else
        print_success "jq is available"
    fi
    
    # Ensure PATH includes jq
    export PATH="$HOME/bin:$PATH"
    
    echo ""
}

# Step 1: Check dependencies
check_dependencies

# Step 2: Check if Anvil is running
print_step "Checking if Anvil is running..."
if ! curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' http://localhost:8545 > /dev/null 2>&1; then
    print_error "Anvil is not running on localhost:8545"
    echo "Please start Anvil with: anvil"
    echo "Then run this script again."
    exit 1
fi
print_success "Anvil is running"

# Step 3: Deploy smart contracts
print_step "Deploying smart contracts..."
cd contracts
export PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
forge script script/DeploySystem.s.sol --rpc-url http://localhost:8545 --broadcast --private-key $PRIVATE_KEY

# Extract contract addresses from the latest deployment
VEHICLE_REGISTRY=$(jq -r '.transactions[] | select(.contractName == "VehicleRegistry") | .contractAddress' broadcast/DeploySystem.s.sol/31337/run-latest.json)
DATA_MARKETPLACE=$(jq -r '.transactions[] | select(.contractName == "DataMarketplace") | .contractAddress' broadcast/DeploySystem.s.sol/31337/run-latest.json)
ACCESS_CONTROL=$(jq -r '.transactions[] | select(.contractName == "AccessControl") | .contractAddress' broadcast/DeploySystem.s.sol/31337/run-latest.json)

cd ..

print_success "Smart contracts deployed:"
echo "  VehicleRegistry: $VEHICLE_REGISTRY"
echo "  DataMarketplace: $DATA_MARKETPLACE"
echo "  AccessControl: $ACCESS_CONTROL"
echo ""

# Step 4: Fund vehicle wallets
print_step "Funding vehicle wallets..."

# Fund vehicle index 0 wallet
VEHICLE_0_WALLET=0x163c8d2c88174953a3b0ad5f60078340ed30691c
cast send $VEHICLE_0_WALLET --value 0.1ether --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://localhost:8545 > /dev/null
print_success "Funded vehicle 0 wallet: $VEHICLE_0_WALLET"

# Fund vehicle index 1 wallet
VEHICLE_1_WALLET=0xfae38ba9cd936f66d412f4fac29fb3ba9dee8975
cast send $VEHICLE_1_WALLET --value 0.1ether --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://localhost:8545 > /dev/null
print_success "Funded vehicle 1 wallet: $VEHICLE_1_WALLET"

# Fund vehicle index 5 wallet
VEHICLE_5_WALLET=0xa6ec3eeeb4d49443580d807d66bbc4dd9de4f1b3
cast send $VEHICLE_5_WALLET --value 0.1ether --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://localhost:8545 > /dev/null
print_success "Funded vehicle 5 wallet: $VEHICLE_5_WALLET"

echo ""

# Step 5: Register vehicles
print_step "Registering vehicles..."

# Register Vehicle 1: Honda Civic 2023
print_step "Registering Vehicle 1: Honda Civic 2023..."
./scripts/run-rust.sh --index 0 --registry-address $VEHICLE_REGISTRY register-vehicle \
    --vin "1HGBH41JXMN109186" \
    --manufacturer "Honda" \
    --model "Civic" \
    --year 2023
print_success "Registered Honda Civic 2023"

# Register Vehicle 2: Toyota Camry 2024
print_step "Registering Vehicle 2: Toyota Camry 2024..."
./scripts/run-rust.sh --index 1 --registry-address $VEHICLE_REGISTRY register-vehicle \
    --vin "2HGBH41JXMN109187" \
    --manufacturer "Toyota" \
    --model "Camry" \
    --year 2024
print_success "Registered Toyota Camry 2024"

# Register Vehicle 3: Ford F-150 2025
print_step "Registering Vehicle 3: Ford F-150 2025..."
./scripts/run-rust.sh --index 5 --registry-address $VEHICLE_REGISTRY register-vehicle \
    --vin "3HGBH41JXMN109189" \
    --manufacturer "Ford" \
    --model "F-150" \
    --year 2025
print_success "Registered Ford F-150 2025"

echo ""

# Step 6: Generate condition reports for all vehicles
print_step "Generating vehicle condition reports..."

./scripts/run-rust.sh --index 0 --registry-address $VEHICLE_REGISTRY sign-report > /dev/null
print_success "Generated condition report for Honda Civic"

./scripts/run-rust.sh --index 1 --registry-address $VEHICLE_REGISTRY sign-report > /dev/null
print_success "Generated condition report for Toyota Camry"

./scripts/run-rust.sh --index 5 --registry-address $VEHICLE_REGISTRY sign-report > /dev/null
print_success "Generated condition report for Ford F-150"

echo ""

# Step 7: List data products on marketplace
print_step "Listing data products on marketplace..."

# List Honda Civic data
./scripts/run-rust.sh --index 0 \
    --registry-address $VEHICLE_REGISTRY \
    --marketplace-address $DATA_MARKETPLACE \
    list-data \
    --data-type "condition_report" \
    --price 500000000000000000 \
    --description "Honda Civic 2023 - Complete vehicle diagnostics and mileage data"
print_success "Listed Honda Civic data (0.5 ETH)"

# List Toyota Camry data
./scripts/run-rust.sh --index 1 \
    --registry-address $VEHICLE_REGISTRY \
    --marketplace-address $DATA_MARKETPLACE \
    list-data \
    --data-type "condition_report" \
    --price 750000000000000000 \
    --description "Toyota Camry 2024 - Battery health, mileage, and maintenance records"
print_success "Listed Toyota Camry data (0.75 ETH)"

# List Ford F-150 data
./scripts/run-rust.sh --index 5 \
    --registry-address $VEHICLE_REGISTRY \
    --marketplace-address $DATA_MARKETPLACE \
    list-data \
    --data-type "condition_report" \
    --price 1000000000000000000 \
    --description "Ford F-150 2025 - Comprehensive truck diagnostics and performance data"
print_success "Listed Ford F-150 data (1.0 ETH)"

echo ""

# Step 8: Display summary
print_success "🎉 Setup Complete!"
echo ""
echo "📋 Summary:"
echo "==========="
echo "📍 Contract Addresses:"
echo "   VehicleRegistry:  $VEHICLE_REGISTRY"
echo "   DataMarketplace:  $DATA_MARKETPLACE"
echo "   AccessControl:    $ACCESS_CONTROL"
echo ""
echo "🚗 Registered Vehicles:"
echo "   1. Honda Civic 2023   (VIN: 1HGBH41JXMN109186) - Wallet: $VEHICLE_0_WALLET"
echo "   2. Toyota Camry 2024  (VIN: 2HGBH41JXMN109187) - Wallet: $VEHICLE_1_WALLET"
echo "   3. Ford F-150 2025    (VIN: 3HGBH41JXMN109189) - Wallet: $VEHICLE_5_WALLET"
echo ""
echo "💰 Marketplace Listings:"
echo "   • Honda Civic data    - 0.5 ETH"
echo "   • Toyota Camry data   - 0.75 ETH"
echo "   • Ford F-150 data     - 1.0 ETH"
echo ""
echo "🌐 Next Steps:"
echo "   • Start the web frontend: ./scripts/run-web.sh"
echo "   • Visit http://localhost:3001 to see the marketplace"
echo "   • All vehicles should appear in the marketplace tab"
echo ""
print_success "Your Vehicle Net system is ready! 🚗💨"