#!/bin/bash

# Launch Next.js frontend with blockchain integration
set -e

handle_error() {
    local exit_code=$?
    local msg="$1"
    if [ -n "$msg" ]; then
        echo "❌ $msg"
    else
        echo "❌ An unexpected error occurred."
    fi
    exit $exit_code
}

trap 'handle_error "Script failed at line $LINENO."' ERR

echo "🌐 Starting Vehicle Network Frontend..."

# Check if config exists
if [ ! -f scripts/config.sh ]; then
    echo "❌ Configuration not found. Please run ./scripts/deploy.sh first"
    exit 1
fi

# Load configuration
if ! source scripts/config.sh; then
    handle_error "Failed to load configuration from scripts/config.sh"
fi

echo "📋 Using configuration:"
echo "RPC URL: $RPC_URL"
echo "Registry: $REGISTRY_ADDRESS"
echo "Marketplace: $MARKETPLACE_ADDRESS"

# Check if web directory exists
if [ ! -d "web" ]; then
    echo "❌ Web directory not found"
    exit 1
fi

# Export environment variables for the frontend
export NEXT_PUBLIC_RPC_URL="$RPC_URL"
export NEXT_PUBLIC_REGISTRY_ADDRESS="$REGISTRY_ADDRESS"
export NEXT_PUBLIC_MARKETPLACE_ADDRESS="$MARKETPLACE_ADDRESS"
export NEXT_PUBLIC_ACCESS_CONTROL_ADDRESS="$ACCESS_CONTROL_ADDRESS"
export NEXT_PUBLIC_CHAIN_ID="31337"

if ! cd web; then
    handle_error "Failed to change directory to 'web'"
fi

# Detect package manager
if command -v bun >/dev/null 2>&1; then
    PKG_MGR="bun"
    INSTALL_CMD="bun update"
    BUILD_CMD="bun run build"
    DEV_CMD="bun run dev"
elif command -v npm >/dev/null 2>&1; then
    PKG_MGR="npm"
    INSTALL_CMD="npm install"
    BUILD_CMD="npm run build"
    DEV_CMD="npm run dev"
else
    echo "❌ Neither bun nor npm is installed. Please install one of them to continue."
    exit 1
fi

# Install dependencies if needed

echo "📦 Installing frontend dependencies with $PKG_MGR..."
if ! $INSTALL_CMD; then
    handle_error "Dependency installation failed with $PKG_MGR"
fi

if ! $BUILD_CMD; then
    handle_error "Frontend build failed with $PKG_MGR"
fi

echo "🚀 Starting Next.js development server with $PKG_MGR..."
echo "Frontend will be available at: http://localhost:3001"
echo "Press Ctrl+C to stop the server"

if ! $DEV_CMD; then
    handle_error "Failed to start the development server with $PKG_MGR"
fi