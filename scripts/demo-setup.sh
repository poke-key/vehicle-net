#!/bin/bash

# Demo Setup Script for Hackathon
# This script helps set up the entire demo environment

echo "🏁 Vehicle Network - Hackathon Demo Setup"
echo "========================================"
echo ""

# Check if we're in the right directory
if [ ! -f "CLAUDE.md" ]; then
    echo "❌ Please run this script from the vehicle-net root directory"
    exit 1
fi

echo "📋 Demo Instructions:"
echo ""
echo "1. Terminal 1 - Start Blockchain (anvil):"
echo "   cd contracts && anvil"
echo ""
echo "2. Terminal 2 - Deploy Contracts (run once):"
echo "   ./scripts/run-contracts.sh"
echo ""
echo "3. Terminal 3-N - Vehicle Nodes (one per vehicle):"
echo "   # Register vehicle first (run once per vehicle):"
echo "   ./scripts/register-vehicle.sh 1 'TESLA001' 'Tesla' 'Model 3' 2023"
echo ""
echo "   # Then run vehicle daemon (continuous):"
echo "   ./scripts/run-vehicle-node.sh 1 'TESLA001'"
echo ""
echo "   # For additional vehicles:"
echo "   ./scripts/register-vehicle.sh 2 'BMW002' 'BMW' 'i4' 2022"
echo "   ./scripts/run-vehicle-node.sh 2 'BMW002'"
echo ""
echo "4. Terminal X - Web Frontend:"
echo "   ./scripts/run-web.sh"
echo ""
echo "🎯 Demo Flow:"
echo "1. Start anvil blockchain"
echo "2. Deploy contracts" 
echo "3. Register 2-3 vehicles"
echo "4. Start vehicle node daemons (each in separate terminal)"
echo "5. Start web frontend"
echo "6. Visit http://localhost:3001 to see live vehicle data!"
echo ""
echo "💡 Each vehicle will continuously transmit condition reports"
echo "💡 The website updates every 5 seconds with real blockchain data"
echo "💡 All data is cryptographically signed by each vehicle's wallet"
echo ""

echo "🚀 Ready to start your hackathon demo!"
echo "Press any key to continue..."
read -n 1 -s