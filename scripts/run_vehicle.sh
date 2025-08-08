#!/bin/bash

# Launch a single vehicle instance
set -e

# Check arguments
if [ $# -lt 3 ]; then
    echo "Usage: ./scripts/run_vehicle.sh <VIN> <MILEAGE> <BATTERY_HEALTH>"
    echo "Example: ./scripts/run_vehicle.sh VIN123ABC 12000 85"
    exit 1
fi

VIN=$1
MILEAGE=$2
BATTERY_HEALTH=$3

echo "🚗 Starting Vehicle: $VIN"

# Check if config exists
if [ ! -f scripts/config.sh ]; then
    echo "❌ Configuration not found. Please run ./scripts/deploy.sh first"
    exit 1
fi

# Load configuration
source scripts/config.sh

echo "📋 Using configuration:"
echo "RPC URL: $RPC_URL"
echo "Registry: $REGISTRY_ADDRESS"
echo "Vehicle: $VIN (${MILEAGE}mi, ${BATTERY_HEALTH}% battery)"

# Check if rust directory exists
if [ ! -d "rust" ]; then
    echo "❌ Rust directory not found"
    exit 1
fi

cd rust

# Build if needed
if [ ! -f "target/release/vehicle-net" ] && [ ! -f "target/debug/vehicle-net" ]; then
    echo "🔨 Building Rust vehicle client..."
    cargo build --release
fi

# Determine which binary to use
BINARY_PATH=""
if [ -f "target/release/vehicle-net" ]; then
    BINARY_PATH="target/release/vehicle-net"
elif [ -f "target/debug/vehicle-net" ]; then
    BINARY_PATH="target/debug/vehicle-net"
else
    echo "❌ Vehicle binary not found. Building..."
    cargo build
    BINARY_PATH="target/debug/vehicle-net"
fi

# Generate unique vehicle index based on VIN hash
VEHICLE_INDEX=$(echo -n "$VIN" | cksum | awk '{print $1 % 100}')

echo "🔧 Vehicle Index: $VEHICLE_INDEX"

# Get the vehicle address that will be generated  
echo "💰 Getting vehicle address..."
TEMP_OUTPUT=$(./$BINARY_PATH --index $VEHICLE_INDEX --rpc-url "$RPC_URL" --registry-address "$REGISTRY_ADDRESS" --marketplace-address "$MARKETPLACE_ADDRESS" --access-control-address "$ACCESS_CONTROL_ADDRESS" balance 2>&1 || true)
VEHICLE_ADDRESS=$(echo "$TEMP_OUTPUT" | grep -o "Address: 0x[a-fA-F0-9]*" | cut -d' ' -f2)
cd ..

if [ -n "$VEHICLE_ADDRESS" ]; then
    echo "🏦 Vehicle Address: $VEHICLE_ADDRESS"
    
    # Check if vehicle address has sufficient balance (need at least 0.1 ETH for transactions)
    BALANCE=$(cast balance --rpc-url "$RPC_URL" "$VEHICLE_ADDRESS" 2>/dev/null || echo "0")
    REQUIRED_BALANCE="100000000000000000" # 0.1 ETH in wei
    
    if [ "$BALANCE" -lt "$REQUIRED_BALANCE" ]; then
        echo "💸 Funding vehicle address with ETH for gas fees..."
        cast send \
            --rpc-url "$RPC_URL" \
            --private-key "$PRIVATE_KEY" \
            "$VEHICLE_ADDRESS" \
            --value 0.5ether > /dev/null 2>&1
        
        if [ $? -eq 0 ]; then
            echo "✅ Vehicle address funded successfully"
        else
            echo "⚠️  Failed to fund vehicle address, attempting to continue..."
        fi
    else
        echo "✅ Vehicle address already has sufficient balance"
    fi
else
    echo "⚠️  Could not determine vehicle address, attempting to continue..."
fi

# First, register the vehicle
echo "📝 Registering vehicle..."
./$BINARY_PATH \
    --index $VEHICLE_INDEX \
    --rpc-url "$RPC_URL" \
    --registry-address "$REGISTRY_ADDRESS" \
    --marketplace-address "$MARKETPLACE_ADDRESS" \
    --access-control-address "$ACCESS_CONTROL_ADDRESS" \
    register-vehicle \
    --vin "$VIN" \
    --manufacturer "AutoGen" \
    --model "Model-$(echo $VIN | tail -c 4)" \
    --year 2023 \
    --fee 0.01

# Check if registration was successful
if [ $? -eq 0 ]; then
    echo "✅ Vehicle registered successfully"
else
    echo "⚠️  Registration failed or vehicle already exists, continuing..."
fi

# Then submit a condition report
echo "📊 Submitting condition report..."
./$BINARY_PATH \
    --index $VEHICLE_INDEX \
    --rpc-url "$RPC_URL" \
    --registry-address "$REGISTRY_ADDRESS" \
    --marketplace-address "$MARKETPLACE_ADDRESS" \
    --access-control-address "$ACCESS_CONTROL_ADDRESS" \
    submit-report \
    --vin "$VIN" \
    --mileage $MILEAGE \
    --battery-health $BATTERY_HEALTH

if [ $? -eq 0 ]; then
    echo "✅ Condition report submitted"
else
    echo "❌ Failed to submit condition report"
fi

# Get vehicle info
echo "ℹ️  Vehicle Information:"
./$BINARY_PATH \
    --index $VEHICLE_INDEX \
    --rpc-url "$RPC_URL" \
    --registry-address "$REGISTRY_ADDRESS" \
    --marketplace-address "$MARKETPLACE_ADDRESS" \
    --access-control-address "$ACCESS_CONTROL_ADDRESS" \
    get-vehicle

echo ""
echo "🎉 Vehicle $VIN is now active on the blockchain!"
echo "💰 Check balance with: $BINARY_PATH --index $VEHICLE_INDEX --rpc-url $RPC_URL balance"
echo "📋 View on frontend: http://localhost:3001"