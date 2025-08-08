#!/bin/bash

# Vehicle Node Daemon Runner
# Usage: ./scripts/run-vehicle-node.sh <vehicle_index> <vin> [mileage] [battery_health] [interval]

set -e

VEHICLE_INDEX=${1:-1}
VIN=${2:-"DEMO${VEHICLE_INDEX}VEHICLE$(date +%s)"}
STARTING_MILEAGE=${3:-$((50000 + VEHICLE_INDEX * 10000))}
STARTING_BATTERY_HEALTH=${4:-$((95 + VEHICLE_INDEX % 5))}
INTERVAL=${5:-15}

echo "🚗 Starting Vehicle Node #${VEHICLE_INDEX}"
echo "VIN: ${VIN}"
echo "Starting Mileage: ${STARTING_MILEAGE}"
echo "Starting Battery Health: ${STARTING_BATTERY_HEALTH}%"
echo "Report Interval: ${INTERVAL} seconds"
echo ""
echo "💡 This vehicle will continuously transmit data to the blockchain"
echo "🔗 Make sure anvil is running and contracts are deployed"
echo ""

cd rust

cargo run -- \
    --index ${VEHICLE_INDEX} \
    daemon \
    --vin ${VIN} \
    --starting-mileage ${STARTING_MILEAGE} \
    --starting-battery-health ${STARTING_BATTERY_HEALTH} \
    --interval ${INTERVAL}