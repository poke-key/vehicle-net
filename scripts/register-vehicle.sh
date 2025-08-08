#!/bin/bash

# Vehicle Registration Helper
# Usage: ./scripts/register-vehicle.sh <vehicle_index> <vin> <manufacturer> <model> <year>

set -e

VEHICLE_INDEX=${1:-1}
VIN=${2:-"DEMO${VEHICLE_INDEX}VEHICLE$(date +%s)"}
MANUFACTURER=${3:-"Tesla"}
MODEL=${4:-"Model 3"}
YEAR=${5:-2023}
FEE=${6:-0.01}

echo "🚗 Registering Vehicle Node #${VEHICLE_INDEX}"
echo "VIN: ${VIN}"
echo "Vehicle: ${YEAR} ${MANUFACTURER} ${MODEL}"
echo "Registration Fee: ${FEE} ETH"
echo ""

cd rust

cargo run -- \
    --index ${VEHICLE_INDEX} \
    register-vehicle \
    --vin ${VIN} \
    --manufacturer ${MANUFACTURER} \
    --model ${MODEL} \
    --year ${YEAR} \
    --fee ${FEE}

echo ""
echo "✅ Vehicle registered! You can now run the daemon with:"
echo "./scripts/run-vehicle-node.sh ${VEHICLE_INDEX} ${VIN}"