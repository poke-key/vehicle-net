#!/bin/bash

# Deploy smart contracts and start Anvil
set -e

echo "🔧 Starting Vehicle Network Deployment..."

# Hardcoded configuration
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
RPC_URL="http://127.0.0.1:8545"

# Kill any existing anvil processes
pkill anvil || true
sleep 2

# Start Anvil in the background for deployment
echo "🚀 Starting Anvil for deployment..."
anvil --host 0.0.0.0 --port 8545 --accounts 3 --balance 10000 --mnemonic "test test test test test test test test test test test junk" > anvil.log 2>&1 &
ANVIL_PID=$!

# Wait for Anvil to start
echo "⏳ Waiting for Anvil to start..."
sleep 3

# Test connection
if ! curl -s -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' $RPC_URL > /dev/null; then
    echo "❌ Failed to connect to Anvil"
    kill $ANVIL_PID
    exit 1
fi

echo "✅ Anvil started successfully for deployment"

# Build contracts
echo "🔨 Building contracts..."
cd contracts
forge build

# Deploy contracts
echo "📋 Deploying contracts..."
export PRIVATE_KEY=$PRIVATE_KEY
DEPLOY_OUTPUT=$(forge script script/DeploySystem.s.sol:DeploySystemScript --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --json)

# Extract contract addresses from the deployment output
REGISTRY_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -o '"VehicleRegistry","address":"[^"]*"' | cut -d'"' -f4 || true)
MARKETPLACE_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -o '"DataMarketplace","address":"[^"]*"' | cut -d'"' -f4 || true)
ACCESS_CONTROL_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -o '"AccessControl","address":"[^"]*"' | cut -d'"' -f4 || true)

# If grep didn't work, try a different approach
if [ -z "$REGISTRY_ADDRESS" ]; then
    REGISTRY_ADDRESS=$(forge script script/DeploySystem.s.sol:DeploySystemScript --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast | grep "VehicleRegistry deployed at:" | awk '{print $4}')
fi

cd ..

echo ""
echo "✅ Deployment Complete!"
echo "════════════════════════════════════════════════"
echo "🌐 RPC URL: $RPC_URL"
echo "🔑 Private Key: $PRIVATE_KEY"
echo "📋 VehicleRegistry: $REGISTRY_ADDRESS"
echo "🏪 DataMarketplace: $MARKETPLACE_ADDRESS"
echo "🔐 AccessControl: $ACCESS_CONTROL_ADDRESS"
echo "⚙️  Anvil PID: $ANVIL_PID"
echo "════════════════════════════════════════════════"

# Save configuration for other scripts
cat > scripts/config.sh << EOF
#!/bin/bash
export RPC_URL="$RPC_URL"
export PRIVATE_KEY="$PRIVATE_KEY"
export REGISTRY_ADDRESS="$REGISTRY_ADDRESS"
export MARKETPLACE_ADDRESS="$MARKETPLACE_ADDRESS"
export ACCESS_CONTROL_ADDRESS="$ACCESS_CONTROL_ADDRESS"
export ANVIL_PID="$ANVIL_PID"
EOF

echo "💾 Configuration saved to scripts/config.sh"
echo "🎯 Use scripts/dev_frontend.sh and scripts/run_vehicle.sh to interact with the system"

# Stop the background anvil and start it in foreground
echo ""
echo "🔄 Stopping background anvil and starting in foreground..."
kill $ANVIL_PID
sleep 2

echo "🚀 Starting Anvil in foreground (keep this terminal open)..."
echo "📋 Anvil will keep running. Press Ctrl+C to stop."
echo ""

# Start anvil in foreground to keep it running
anvil --host 0.0.0.0 --port 8545 --accounts 3 --balance 10000 --mnemonic "test test test test test test test test test test test junk"