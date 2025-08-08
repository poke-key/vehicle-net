#!/bin/bash

# Terminal 3+: Vehicle Node Operations
# This terminal handles vehicle registration and data transmission
# Usage: ./scripts/terminal-vehicle.sh <vehicle_index> [command] [options]

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

# Function to check if rust directory exists
check_rust_directory() {
    if [ ! -d "rust" ]; then
        print_error "Rust directory not found. Please run this script from the project root."
        exit 1
    fi
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

# Function to register a vehicle
register_vehicle() {
    local vehicle_index=$1
    local vin=$2
    local manufacturer=${3:-"Tesla"}
    local model=${4:-"Model 3"}
    local year=${5:-2023}
    local fee=${6:-0.01}
    
    print_section "Registering Vehicle"
    print_info "Vehicle Index: $vehicle_index"
    print_info "VIN: $vin"
    print_info "Vehicle: $year $manufacturer $model"
    print_info "Registration Fee: $fee ETH"
    echo ""
    
    check_rust_directory
    cd rust
    
    cargo run -- \
        --index $vehicle_index \
        register-vehicle \
        --vin "$vin" \
        --manufacturer "$manufacturer" \
        --model "$model" \
        --year $year \
        --fee $fee
    
    cd ..
    
    print_success "Vehicle registered successfully!"
    print_info "You can now start the vehicle daemon with:"
    print_info "$0 $vehicle_index daemon --vin $vin"
}

# Function to start vehicle daemon
start_daemon() {
    local vehicle_index=$1
    local vin=$2
    local starting_mileage=${3:-$((50000 + vehicle_index * 10000))}
    local starting_battery=${4:-$((95 + vehicle_index % 5))}
    local interval=${5:-15}
    
    print_section "Starting Vehicle Daemon"
    print_info "Vehicle Index: $vehicle_index"
    print_info "VIN: $vin"
    print_info "Starting Mileage: $starting_mileage"
    print_info "Starting Battery Health: $starting_battery%"
    print_info "Report Interval: $interval seconds"
    echo ""
    
    # Check if anvil is running
    if ! check_anvil; then
        print_error "Anvil is not running. Please start anvil first:"
        print_info "./scripts/terminal-contracts.sh start-anvil"
        exit 1
    fi
    
    print_success "Anvil is running on localhost:8545"
    print_info "This vehicle will continuously transmit data to the blockchain"
    print_info "Press Ctrl+C to stop the daemon"
    echo ""
    
    check_rust_directory
    cd rust
    
    # Start the daemon with infinite loop
    cargo run -- \
        --index $vehicle_index \
        daemon \
        --vin "$vin" \
        --starting-mileage $starting_mileage \
        --starting-battery-health $starting_battery \
        --interval $interval
    
    cd ..
}

# Function to get vehicle information
get_vehicle_info() {
    local vehicle_index=$1
    
    print_section "Vehicle Information"
    print_info "Vehicle Index: $vehicle_index"
    echo ""
    
    check_rust_directory
    cd rust
    
    cargo run -- --index $vehicle_index info
    
    cd ..
}

# Function to check vehicle balance
check_balance() {
    local vehicle_index=$1
    
    print_section "Vehicle Balance"
    print_info "Vehicle Index: $vehicle_index"
    echo ""
    
    check_rust_directory
    cd rust
    
    cargo run -- --index $vehicle_index balance
    
    cd ..
}

# Function to sign a report (offline)
sign_report() {
    local vehicle_index=$1
    local vin=$2
    local mileage=${3:-50000}
    local battery_health=${4:-95}
    
    print_section "Signing Vehicle Report"
    print_info "Vehicle Index: $vehicle_index"
    print_info "VIN: $vin"
    print_info "Mileage: $mileage"
    print_info "Battery Health: $battery_health%"
    echo ""
    
    check_rust_directory
    cd rust
    
    cargo run -- \
        --index $vehicle_index \
        sign-report \
        --vin "$vin" \
        --mileage $mileage \
        --battery-health $battery_health
    
    cd ..
}

# Function to list data on marketplace
list_data() {
    local vehicle_index=$1
    local data_type=${2:-"GPS"}
    local price=${3:-0.001}
    local description=${4:-"Real-time vehicle data"}
    
    print_section "Listing Data on Marketplace"
    print_info "Vehicle Index: $vehicle_index"
    print_info "Data Type: $data_type"
    print_info "Price: $price ETH"
    print_info "Description: $description"
    echo ""
    
    check_rust_directory
    cd rust
    
    cargo run -- \
        --index $vehicle_index \
        list-data \
        --data-type "$data_type" \
        --price $price \
        --description "$description"
    
    cd ..
}

# Function to show vehicle status
show_status() {
    local vehicle_index=$1
    
    print_section "Vehicle Status"
    print_info "Vehicle Index: $vehicle_index"
    echo ""
    
    # Check if anvil is running
    if check_anvil; then
        print_success "Blockchain is running (anvil)"
    else
        print_error "Blockchain is not running (anvil)"
    fi
    
    # Check if contracts are built
    if [ -d "contracts/out" ]; then
        print_success "Contracts are built"
    else
        print_warning "Contracts are not built"
    fi
    
    # Check if rust directory exists
    if [ -d "rust" ]; then
        print_success "Rust application ready"
    else
        print_error "Rust directory not found"
    fi
    
    echo ""
    print_info "Available commands:"
    print_info "  $0 $vehicle_index register <vin> [manufacturer] [model] [year] [fee]"
    print_info "  $0 $vehicle_index daemon <vin> [mileage] [battery] [interval]"
    print_info "  $0 $vehicle_index info"
    print_info "  $0 $vehicle_index balance"
    print_info "  $0 $vehicle_index sign <vin> [mileage] [battery]"
    print_info "  $0 $vehicle_index list <data_type> [price] [description]"
}

# Function to show help
show_help() {
    local vehicle_index=${1:-1}
    
    print_header "Terminal 3+: Vehicle Node Operations"
    print_info "This terminal handles vehicle registration and data transmission"
    echo ""
    
    echo "Usage: $0 <vehicle_index> [command] [options]"
    echo ""
    echo "Vehicle Index:"
    echo "  <vehicle_index>  - The vehicle index (1, 2, 3, etc.)"
    echo ""
    echo "Commands:"
    echo "  register <vin> [manufacturer] [model] [year] [fee]"
    echo "                    - Register a new vehicle on the blockchain"
    echo "  daemon <vin> [mileage] [battery] [interval]"
    echo "                    - Start vehicle daemon for continuous data transmission"
    echo "  info              - Get vehicle information"
    echo "  balance           - Check vehicle balance"
    echo "  sign <vin> [mileage] [battery]"
    echo "                    - Sign a vehicle report (offline)"
    echo "  list <data_type> [price] [description]"
    echo "                    - List data on the marketplace"
    echo "  status            - Show vehicle and system status"
    echo "  help              - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 1 register TESLA001 Tesla Model3 2023 0.01"
    echo "  $0 1 daemon TESLA001 50000 95 15"
    echo "  $0 1 info"
    echo "  $0 1 balance"
    echo "  $0 1 sign TESLA001 50003 95"
    echo "  $0 1 list GPS 0.001 'Real-time GPS data'"
    echo ""
    print_info "💡 For the demo, run: $0 1 register TESLA001 && $0 1 daemon TESLA001"
}

# Main function
main() {
    # Check if vehicle index is provided
    if [ $# -eq 0 ]; then
        show_help
        exit 1
    fi
    
    local vehicle_index=$1
    shift
    
    # Validate vehicle index
    if ! [[ "$vehicle_index" =~ ^[0-9]+$ ]]; then
        print_error "Vehicle index must be a number"
        exit 1
    fi
    
    # Parse command
    case "${1:-help}" in
        "register")
            shift
            register_vehicle $vehicle_index "$@"
            ;;
        "daemon")
            shift
            start_daemon $vehicle_index "$@"
            ;;
        "info")
            get_vehicle_info $vehicle_index
            ;;
        "balance")
            check_balance $vehicle_index
            ;;
        "sign")
            shift
            sign_report $vehicle_index "$@"
            ;;
        "list")
            shift
            list_data $vehicle_index "$@"
            ;;
        "status")
            show_status $vehicle_index
            ;;
        "help"|*)
            show_help $vehicle_index
            ;;
    esac
}

# Run main function
main "$@"
