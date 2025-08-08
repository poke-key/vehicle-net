#!/bin/bash

# Deploy simplified contracts
echo "🚀 Deploying SimpleVehicleNodes contract..."

cd contracts

# Start anvil in background if not already running
if ! pgrep -f "anvil" > /dev/null; then
    echo "Starting local Anvil node..."
    anvil --host 0.0.0.0 --port 8545 &
    ANVIL_PID=$!
    sleep 3
    echo "Anvil started with PID: $ANVIL_PID"
else
    echo "Anvil is already running"
fi

# Deploy the contract
echo "Deploying SimpleVehicleNodes..."
forge create src/SimpleVehicleNodes.sol:SimpleVehicleNodes \
    --rpc-url http://localhost:8545 \
    --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
    --json > deployment.json

if [ $? -eq 0 ]; then
    echo "✅ Contract deployed successfully!"
    
    # Extract deployed address
    DEPLOYED_ADDRESS=$(cat deployment.json | jq -r '.deployedTo')
    echo "📍 Contract Address: $DEPLOYED_ADDRESS"
    
    # Save address for easy reference
    echo "$DEPLOYED_ADDRESS" > contract_address.txt
    
    echo "🎉 Simple contract deployment complete!"
    echo "Use this address in your Rust and web applications."
else
    echo "❌ Contract deployment failed"
    exit 1
fi