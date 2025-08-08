#!/bin/bash

# Terminal 2: Web Application Development
# This terminal handles the web interface building and development server

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

# Function to check if web directory exists
check_web_directory() {
    if [ ! -d "web" ]; then
        print_error "Web directory not found. Please run this script from the project root."
        exit 1
    fi
}

# Function to detect package manager
detect_package_manager() {
    if command -v bun &> /dev/null; then
        echo "bun"
    elif command -v yarn &> /dev/null; then
        echo "yarn"
    elif command -v npm &> /dev/null; then
        echo "npm"
    else
        echo "none"
    fi
}

# Function to install dependencies
install_dependencies() {
    print_section "Installing Dependencies"
    
    check_web_directory
    cd web
    
    local pkg_manager=$(detect_package_manager)
    
    case $pkg_manager in
        "bun")
            print_info "Using Bun to install/update dependencies..."
            bun update
            ;;
        "yarn")
            print_info "Using Yarn to install dependencies..."
            yarn install
            ;;
        "npm")
            print_info "Using npm to install dependencies..."
            npm install
            ;;
        "none")
            print_error "No package manager (bun, yarn, or npm) found in PATH"
            print_info "Please install one of: bun, yarn, or npm"
            exit 1
            ;;
    esac
    
    cd ..
    print_success "Dependencies installed successfully"
}

# Function to build the web application
build_web() {
    print_section "Building Web Application"
    
    check_web_directory
    cd web
    
    local pkg_manager=$(detect_package_manager)
    
    case $pkg_manager in
        "bun")
            print_info "Using Bun to build the application..."
            bun run build
            ;;
        "yarn")
            print_info "Using Yarn to build the application..."
            yarn build
            ;;
        "npm")
            print_info "Using npm to build the application..."
            npm run build
            ;;
    esac
    
    cd ..
    print_success "Web application built successfully"
}

# Function to start development server
start_dev_server() {
    print_section "Starting Development Server"
    
    check_web_directory
    cd web
    
    local pkg_manager=$(detect_package_manager)
    
    print_info "Starting web application..."
    print_info "The application will be available at: http://localhost:3001"
    print_info "Press Ctrl+C to stop the server"
    echo ""
    
    # Show current directory and package.json
    echo "Current directory: $(pwd)"
    echo "Checking for package.json: $(ls -la package.json 2>/dev/null || echo 'package.json not found')"
    echo ""
    
    case $pkg_manager in
        "bun")
            print_info "Found Bun: $(which bun)"
            print_info "Using Bun to run dev server..."
            bun run dev
            ;;
        "yarn")
            print_info "Found Yarn: $(which yarn)"
            print_info "Using Yarn to run dev server..."
            yarn dev
            ;;
        "npm")
            print_info "Found npm: $(which npm)"
            print_info "Using npm to run dev server..."
            npm run dev
            ;;
    esac
}

# Function to check web application status
check_status() {
    print_section "Web Application Status"
    
    # Check if web directory exists
    if [ ! -d "web" ]; then
        print_error "Web directory not found"
        return 1
    fi
    
    print_success "Web directory exists"
    
    # Check package.json
    if [ -f "web/package.json" ]; then
        print_success "package.json found"
    else
        print_error "package.json not found in web directory"
        return 1
    fi
    
    # Check node_modules
    if [ -d "web/node_modules" ]; then
        print_success "Dependencies installed (node_modules exists)"
    else
        print_warning "Dependencies not installed (node_modules missing)"
    fi
    
    # Check package manager
    local pkg_manager=$(detect_package_manager)
    if [ "$pkg_manager" != "none" ]; then
        print_success "Package manager found: $pkg_manager"
    else
        print_error "No package manager found"
    fi
    
    # Check if dev server is running
    if curl -s http://localhost:3001 > /dev/null 2>&1; then
        print_success "Development server is running on http://localhost:3001"
    else
        print_warning "Development server is not running"
    fi
}

# Function to clean build artifacts
clean_build() {
    print_section "Cleaning Build Artifacts"
    
    check_web_directory
    cd web
    
    # Remove common build directories
    if [ -d ".next" ]; then
        rm -rf .next
        print_success "Removed .next directory"
    fi
    
    if [ -d "dist" ]; then
        rm -rf dist
        print_success "Removed dist directory"
    fi
    
    if [ -d "build" ]; then
        rm -rf build
        print_success "Removed build directory"
    fi
    
    cd ..
    print_success "Build artifacts cleaned"
}

# Main function
main() {
    print_header "Terminal 2: Web Application Development"
    print_info "This terminal handles the web interface building and development"
    echo ""
    
    # Parse command line arguments
    case "${1:-help}" in
        "install")
            install_dependencies
            ;;
        "build")
            install_dependencies
            build_web
            ;;
        "dev"|"start")
            install_dependencies
            start_dev_server
            ;;
        "status")
            check_status
            ;;
        "clean")
            clean_build
            ;;
        "help"|*)
            echo "Usage: $0 [command]"
            echo ""
            echo "Commands:"
            echo "  install  - Install/update web dependencies"
            echo "  build    - Build the web application for production"
            echo "  dev      - Start development server"
            echo "  start    - Alias for dev (start development server)"
            echo "  status   - Check web application status"
            echo "  clean    - Clean build artifacts"
            echo "  help     - Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 dev        # Start development server"
            echo "  $0 build      # Build for production"
            echo "  $0 status     # Check current status"
            echo ""
            print_info "💡 For the demo, run: $0 dev"
            ;;
    esac
}

# Run main function
main "$@"
