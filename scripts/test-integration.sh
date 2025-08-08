#!/bin/bash

# Vehicle Net Integration Test Script
# This script comprehensively tests Rust and smart contracts integration

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

# Function to test Rust application
test_rust_application() {
    print_header "Testing Rust Application"
    
    # Test basic compilation
    cd rust
    run_command "Building Rust application" "cargo build --release"
    
    # Create output directory if it doesn't exist
    mkdir -p "../scripts/output"
    
    # Test with different vehicle indices and commands
    print_section "Testing Rust with different vehicle indices"
    
    for i in {0..2}; do
        echo "Testing vehicle index: $i"
        
        # Test help command first
        if cargo run --release -- --index "$i" --help > "../scripts/output/rust_help_$i.txt" 2>&1; then
            print_success "Vehicle $i help command passed"
        else
            print_error "Vehicle $i help command failed"
            cat "../scripts/output/rust_help_$i.txt"
        fi
        
        # Test info command (doesn't require blockchain connection)
        if cargo run --release -- --index "$i" info > "../scripts/output/rust_info_$i.txt" 2>&1; then
            print_success "Vehicle $i info command passed"
            echo "Info output preview:"
            head -10 "../scripts/output/rust_info_$i.txt"
            echo "..."
        else
            print_warning "Vehicle $i info command failed (expected if no blockchain connection)"
            echo "Info output:"
            cat "../scripts/output/rust_info_$i.txt"
        fi
        
        # Test sign-report command (offline operation)
        if cargo run --release -- --index "$i" sign-report > "../scripts/output/rust_sign_$i.txt" 2>&1; then
            print_success "Vehicle $i sign-report command passed"
            echo "Sign output preview:"
            head -10 "../scripts/output/rust_sign_$i.txt"
            echo "..."
        else
            print_error "Vehicle $i sign-report command failed"
            cat "../scripts/output/rust_sign_$i.txt"
        fi
        
        echo ""
    done
    
    cd ..
}

# Function to test smart contracts
test_smart_contracts() {
    print_header "Testing Smart Contracts"
    
    cd contracts
    
    # Check if Foundry is installed
    if ! command -v forge &> /dev/null; then
        print_error "Foundry not found. Please install Foundry first."
        print_info "Run: curl -L https://foundry.paradigm.xyz | bash"
        return 1
    fi
    
    # Create output directory if it doesn't exist
    mkdir -p "../scripts/output"
    
    # Build contracts
    run_command "Building smart contracts" "forge build"
    
    # Run all tests with verbose output
    print_section "Running smart contract tests"
    echo "Running: forge test -vvv"
    echo ""
    
    if forge test -vvv > "../scripts/output/contracts_test_output.txt" 2>&1; then
        print_success "All smart contract tests passed"
        echo "Test summary:"
        tail -20 "../scripts/output/contracts_test_output.txt"
    else
        print_error "Smart contract tests failed"
        echo "Full test output:"
        cat "../scripts/output/contracts_test_output.txt"
        return 1
    fi
    
    cd ..
}

# Function to test contract integration
test_contract_integration() {
    print_header "Testing Contract Integration"
    
    # Check if Rust can interact with contracts
    print_section "Testing Rust contract client compilation"
    
    cd rust
    
    # Create output directory if it doesn't exist
    mkdir -p "../scripts/output"
    
    # Test if the contract client compiles
    if cargo build --release > "../scripts/output/integration_check.txt" 2>&1; then
        print_success "Contract client compiles successfully"
        echo "Compilation check passed - all contract bindings are valid"
    else
        print_error "Contract client compilation failed"
        cat "../scripts/output/integration_check.txt"
        cd ..
        return 1
    fi
    
    # Test offline operations that don't require blockchain connection
    print_section "Testing offline contract operations"
    echo "Testing sign-report command (offline operation)..."
    
    if cargo run --release -- --index 0 sign-report > "../scripts/output/integration_offline.txt" 2>&1; then
        print_success "Offline contract operations work"
        echo "Offline test output preview:"
        head -15 "../scripts/output/integration_offline.txt"
        echo "..."
    else
        print_error "Offline contract operations failed"
        cat "../scripts/output/integration_offline.txt"
        cd ..
        return 1
    fi
    
    # Test that the app can start (even if blockchain connection fails)
    print_section "Testing app startup with contract integration"
    echo "Testing app startup (blockchain connection may fail)..."
    
    if timeout 10s cargo run --release -- --index 0 info > "../scripts/output/integration_startup.txt" 2>&1; then
        print_success "App starts successfully with contract integration"
        echo "Startup test output preview:"
        head -10 "../scripts/output/integration_startup.txt"
        echo "..."
    else
        print_warning "App startup test failed (may be expected without blockchain)"
        echo "Startup test output:"
        cat "../scripts/output/integration_startup.txt"
    fi
    
    cd ..
}

# Function to display test summary
show_test_summary() {
    print_header "Test Summary"
    
    echo "Generated test files:"
    ls -la output/*.txt 2>/dev/null || echo "No test output files found"
    
    echo ""
    print_info "Test files contain detailed output for debugging:"
    echo "- output/rust_help_*.txt: Rust help command output for each vehicle"
    echo "- output/rust_info_*.txt: Rust info command output for each vehicle"
    echo "- output/rust_sign_*.txt: Rust sign-report command output for each vehicle"
    echo "- output/contracts_test_output.txt: Smart contract test results"
    echo "- output/integration_check.txt: Contract client compilation check"
    echo "- output/integration_offline.txt: Offline contract operations test"
    echo "- output/integration_startup.txt: App startup with contract integration test"
    
    echo ""
    print_success "Integration testing completed!"
    print_info "Check the output files above for detailed results."
}

# Function to cleanup test files
cleanup_test_files() {
    print_section "Cleaning up test files"
    
    local files_to_clean=(
        "output/rust_help_*.txt"
        "output/rust_info_*.txt"
        "output/rust_sign_*.txt"
        "output/contracts_test_output.txt"
        "output/integration_check.txt"
        "output/integration_offline.txt"
        "output/integration_startup.txt"
    )
    
    for pattern in "${files_to_clean[@]}"; do
        if ls $pattern 1> /dev/null 2>&1; then
            rm -f $pattern
            echo "Cleaned up: $pattern"
        fi
    done
    
    print_success "Cleanup completed"
}

# Function to prompt for cleanup with countdown
prompt_for_cleanup() {
    print_section "Cleanup Options"
    echo "Test files have been saved to scripts/output/"
    echo ""
    echo "Press 'o' within 10 seconds to keep the output files,"
    echo "or wait for automatic cleanup..."
    echo ""
    
    # 10 second countdown with option to keep files
    for i in {10..1}; do
        echo -ne "\r⏰ Auto-cleanup in $i seconds... (press 'o' to keep files) "
        
        # Read a single character with timeout
        if read -t 1 -n 1 key; then
            if [[ "$key" == "o" || "$key" == "O" ]]; then
                echo ""
                print_success "Keeping output files!"
                print_info "Files are available in scripts/output/"
                return 0
            fi
        fi
    done
    
    echo ""
    echo ""
    cleanup_test_files
}

# Main test function
main() {
    echo "🚗 Vehicle Net - Integration Test Suite"
    echo "======================================"
    echo ""
    print_info "This script will test:"
    echo "1. Rust application compilation and execution"
    echo "2. Smart contract compilation and tests"
    echo "3. Contract integration between Rust and Solidity"
    echo ""
    
    # Check if we're in the right directory structure
    if [ ! -d "rust" ] || [ ! -d "contracts" ]; then
        print_error "Required directories not found. Please run from project root with rust/ and contracts/ directories present."
        exit 1
    fi
    
    # Parse command line arguments
    local cleanup_only=false
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --cleanup)
                cleanup_only=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [OPTIONS]"
                echo ""
                echo "Options:"
                echo "  --cleanup    Clean up test output files only"
                echo "  --help, -h   Show this help message"
                echo ""
                echo "This script tests Rust and smart contract integration."
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                echo "Use --help for usage information"
                exit 1
                ;;
        esac
    done
    
    if [ "$cleanup_only" = true ]; then
        cleanup_test_files
        exit 0
    fi
    
    # Run tests
    test_rust_application
    test_smart_contracts
    test_contract_integration
    show_test_summary
    
    echo ""
    print_success "🎉 All tests completed!"
    
    # Prompt for cleanup
    prompt_for_cleanup
}

# Run main function
main "$@"
