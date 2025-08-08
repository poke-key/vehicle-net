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
    
    # Store the original directory
    local original_dir=$(pwd)
    
    # Set up trap to return to original directory on exit
    trap 'cd "$original_dir"' EXIT
    
    # Initialize and update submodules from project root
    print_section "Initializing submodules"
    run_command "Initializing submodules" "git submodule update --init --recursive --force"
    
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

    
    # Check and update submodules
    print_section "Checking submodules"
    if [ -d "lib/forge-std" ] && [ -d "lib/openzeppelin-contracts" ]; then
        print_info "Dependencies found in lib/ directory (submodules)"
    else
        print_error "Required dependencies not found in lib/ directory"
        print_info "Please ensure submodules are initialized: git submodule update --init --recursive"
        exit 1
    fi
    
    # Debug: Check what was installed
    print_section "Checking installed dependencies"
    if [ -d "lib" ]; then
        echo "Contents of lib directory:"
        ls -la lib/
        if [ -d "lib/forge-std" ]; then
            echo "forge-std contents:"
            ls -la lib/forge-std/
            if [ -d "lib/forge-std/src" ]; then
                echo "forge-std/src contents:"
                ls -la lib/forge-std/src/
            fi
        fi
        if [ -d "lib/openzeppelin-contracts" ]; then
            echo "openzeppelin-contracts contents:"
            ls -la lib/openzeppelin-contracts/
        fi
    else
        echo "lib directory does not exist!"
    fi
    echo ""
    
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
    
    # Return to original directory
    cd "$original_dir"
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
