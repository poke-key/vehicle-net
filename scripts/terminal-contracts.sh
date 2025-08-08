#!/bin/bash

# Terminal 1: Smart Contract Building and Deployment
# This terminal handles all blockchain contract operations

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_header() {
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${PURPLE}================================${NC}"
}

print_section() {
    echo -e "${CYAN}--- $1 ---${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# Function to run command and capture output
run_command() {
    local description="$1"
    local command="$2"
    
    print_section "$description"
    echo "Running: $command"
    echo ""
    
    if eval "$command"; then
        print_success "$description completed successfully"
    else
        print_error "$description failed"
        return 1
    fi
    echo ""
}

# Function to check if anvil is running
check_anvil() {
    if curl -s -X POST -H "Content-Type: application/json" \
        --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
        http://localhost:8545 > /dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Function to start anvil
start_anvil() {
    print_section "Starting Anvil Blockchain"
    print_info "Starting local Ethereum blockchain..."
    print_info "This will run in the background"
    echo ""
    
    # Start anvil in background
    cd contracts
    nohup anvil > ../scripts/output/anvil.log 2>&1 &
    ANVIL_PID=$!
    echo $ANVIL_PID > ../scripts/output/anvil.pid
    
    # Wait a moment for anvil to start
    sleep 3
    
    if check_anvil; then
        print_success "Anvil started successfully (PID: $ANVIL_PID)"
        print_info "Anvil logs: scripts/output/anvil.log"
        print_info "To stop anvil: kill $ANVIL_PID"
    else
        print_error "Failed to start anvil"
        return 1
    fi
    
    cd ..
}

# Function to build contracts
build_contracts() {
    print_section "Building Smart Contracts"
    
    # Initialize submodules
    run_command "Initializing submodules" "git submodule update --init --recursive --force"
    
    # Build contracts
    cd contracts
    run_command "Building smart contracts" "forge build"
    cd ..
    
    print_success "🎉 Smart contract build completed!"
    print_info "Build artifacts location: contracts/out/"
    print_info "Built contracts: AccessControl, DataMarketplace, VehicleRegistry"
}

# Function to deploy contracts
deploy_contracts() {
    print_section "Deploying Smart Contracts"
    
    # Check if anvil is running
    if ! check_anvil; then
        print_error "Anvil is not running. Starting anvil first..."
        start_anvil
    fi
    
    print_success "Anvil is running on localhost:8545"
    
    # Set up deployment environment
    export PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    export RPC_URL="http://localhost:8545"
    
    print_info "Using default anvil account for deployment"
    print_info "Private Key: ${PRIVATE_KEY:0:10}..."
    print_info "RPC URL: $RPC_URL"
    echo ""
    
    # Deploy contracts
    cd contracts
    run_command "Deploying smart contracts" "forge script script/DeploySystem.s.sol --rpc-url $RPC_URL --broadcast --verify"
    cd ..
    
    print_success "🎉 Smart contracts deployed successfully!"
    print_info "Contracts are now available on the local blockchain"
    print_info "You can now register vehicles and start vehicle nodes"
}

# Function to stop anvil
stop_anvil() {
    if [ -f "scripts/output/anvil.pid" ]; then
        ANVIL_PID=$(cat scripts/output/anvil.pid)
        if kill -0 $ANVIL_PID 2>/dev/null; then
            print_section "Stopping Anvil"
            kill $ANVIL_PID
            rm scripts/output/anvil.pid
            print_success "Anvil stopped"
        else
            print_warning "Anvil process not found"
        fi
    else
        print_warning "No anvil PID file found"
    fi
}

# Function to show status
show_status() {
    print_section "Blockchain Status"
    
    if check_anvil; then
        print_success "Anvil is running on localhost:8545"
        
        # Get block number
        BLOCK_NUMBER=$(curl -s -X POST -H "Content-Type: application/json" \
            --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
            http://localhost:8545 | grep -o '"result":"[^"]*"' | cut -d'"' -f4)
        
        if [ ! -z "$BLOCK_NUMBER" ]; then
            DECIMAL_BLOCK=$(printf "%d" "$BLOCK_NUMBER")
            print_info "Current block: $DECIMAL_BLOCK"
        fi
    else
        print_error "Anvil is not running"
    fi
    
    # Check if contracts are built
    if [ -d "contracts/out" ]; then
        print_success "Contracts are built"
    else
        print_warning "Contracts are not built"
    fi
}

# Main function
main() {
    # Create output directory
    mkdir -p scripts/output
    
    print_header "Terminal 1: Smart Contract Operations"
    print_info "This terminal handles blockchain contract building and deployment"
    echo ""
    
    # Parse command line arguments
    case "${1:-help}" in
        "start-anvil")
            start_anvil
            ;;
        "build")
            build_contracts
            ;;
        "deploy")
            deploy_contracts
            ;;
        "build-and-deploy")
            build_contracts
            deploy_contracts
            ;;
        "stop-anvil")
            stop_anvil
            ;;
        "status")
            show_status
            ;;
        "help"|*)
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  start-anvil      - Start the local blockchain (anvil)"
            echo "  build            - Build smart contracts"
            echo "  deploy           - Deploy contracts to local blockchain"
            echo "  build-and-deploy - Build and deploy contracts"
            echo "  stop-anvil       - Stop the local blockchain"
            echo "  status           - Show blockchain and contract status"
            echo "  help             - Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 build-and-deploy  # Build and deploy contracts"
            echo "  $0 status            # Check current status"
            echo ""
            print_info "💡 For the demo, run: $0 build-and-deploy"
            ;;
    esac
}

# Run main function
main "$@"
