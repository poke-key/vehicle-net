#!/bin/bash

# Vehicle Net - Smart Contract Build Script
# This script builds the smart contracts using Foundry

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

# Main build function
main() {
    print_header "Vehicle Net - Smart Contract Build"
    
    # Check if we're in the right directory structure
    if [ ! -d "contracts" ]; then
        print_error "Required 'contracts' directory not found. Please run from project root."
        exit 1
    fi
    
    # Change to contracts directory
    cd contracts
    
    # Check if Foundry is installed
    if ! command -v forge &> /dev/null; then
        print_error "Foundry not found. Please install Foundry first."
        print_info "Run: curl -L https://foundry.paradigm.xyz | bash"
        print_info "Then: foundryup"
        exit 1
    fi
    
    print_info "Foundry version: $(forge --version)"
    echo ""
    
    # Install dependencies if needed
    if [ ! -d "lib" ] || [ -z "$(ls -A lib 2>/dev/null)" ]; then
        print_section "Installing dependencies"
        run_command "Installing Foundry dependencies" "forge install"
    else
        print_info "Dependencies already installed"
        echo ""
    fi
    
    # Build contracts
    run_command "Building smart contracts" "forge build"
    
    # Show build output
    print_section "Build Output"
    echo "Contracts built successfully!"
    echo ""
    echo "Build artifacts location: contracts/out/"
    echo ""
    
    # List built contracts
    if [ -d "out" ]; then
        echo "Built contracts:"
        for contract in out/*.sol/*.json; do
            if [ -f "$contract" ]; then
                contract_name=$(basename "$contract" .json)
                echo "  - $contract_name"
            fi
        done
    fi
    
    echo ""
    print_success "🎉 Smart contract build completed!"
    
    # Return to root directory
    cd ..
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --help, -h   Show this help message"
            echo ""
            echo "This script builds the smart contracts using Foundry."
            exit 0
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Run main function
main "$@"
