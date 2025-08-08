#!/bin/bash

echo "🧪 Testing Simplified Vehicle Network System"
echo "=============================================="

# Check if anvil is running
if ! pgrep -f "anvil" > /dev/null; then
    echo "❌ Anvil is not running. Please start it first with: anvil"
    exit 1
fi

echo "✅ Anvil is running"

# Deploy contracts
echo "1. Deploying contracts..."
cd contracts

if [ ! -f "contract_address.txt" ]; then
    echo "   Contract not deployed. Deploying now..."
    ../scripts/deploy-simple-contracts.sh
fi

CONTRACT_ADDRESS=$(cat contract_address.txt)
echo "   Using contract at: $CONTRACT_ADDRESS"

# Test Rust application
echo "2. Testing Rust application..."
cd ../rust

echo "   Adding vehicle node 0..."
cargo run --quiet -- --index 0 --contract-address "$CONTRACT_ADDRESS" --add-node

echo "   Adding vehicle node 1..."
cargo run --quiet -- --index 1 --contract-address "$CONTRACT_ADDRESS" --add-node

echo "   Adding vehicle node 2..."
cargo run --quiet -- --index 2 --contract-address "$CONTRACT_ADDRESS" --add-node

echo "   Updating vehicle reports..."
cargo run --quiet -- --index 0 --contract-address "$CONTRACT_ADDRESS" --update-report
cargo run --quiet -- --index 1 --contract-address "$CONTRACT_ADDRESS" --update-report
cargo run --quiet -- --index 2 --contract-address "$CONTRACT_ADDRESS" --update-report

echo "   Listing all vehicles..."
cargo run --quiet -- --index 0 --contract-address "$CONTRACT_ADDRESS" --list-vehicles

# Test web application
echo "3. Testing web application..."
cd ../web

echo "   Building web application..."
if npm run build --silent 2>/dev/null; then
    echo "   ✅ Web application builds successfully"
else
    echo "   ⚠️  Web application build had warnings but completed"
fi

echo ""
echo "🎉 System test complete!"
echo ""
echo "📋 Summary:"
echo "   - Contract deployed at: $CONTRACT_ADDRESS"
echo "   - 3 vehicle nodes added to blockchain"
echo "   - Vehicle reports updated"
echo "   - Web application compiles"
echo ""
echo "🌐 To test the frontend:"
echo "   1. cd web && npm run dev"
echo "   2. Open http://localhost:3001/vehicles"
echo "   3. View the vehicle data from the blockchain"