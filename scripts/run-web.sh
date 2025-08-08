#!/bin/bash

# Vehicle Net Web Application Runner
# This script runs the web application from the root directory

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🌐 Vehicle Net - Web Application${NC}"
echo "=================================="

# Check if we're in the right directory
if [ ! -d "web" ]; then
    echo "Error: 'web' directory not found. Please run this script from the project root."
    exit 1
fi

# Check if dependencies are installed
if [ ! -d "web/node_modules" ]; then
    echo "Dependencies not found. Installing..."
    cd web
    
    # Detect package manager
    if command -v bun &> /dev/null; then
        echo "Using Bun to install dependencies..."
        bun install
    elif command -v yarn &> /dev/null; then
        echo "Using Yarn to install dependencies..."
        yarn install
    else
        echo "Using npm to install dependencies..."
        npm install
    fi
    
    cd ..
fi

# Change to web directory and start the application
cd web

echo -e "${GREEN}Starting web application...${NC}"
echo "The application will be available at: http://localhost:3001"
echo "Press Ctrl+C to stop the server"
echo ""

# Detect package manager and run the dev server
if command -v bun &> /dev/null; then
    bun run dev
elif command -v yarn &> /dev/null; then
    yarn dev
else
    npm run dev
fi
