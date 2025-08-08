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

# Always install dependencies to ensure they're up to date
echo "Installing/updating dependencies..."
cd web

# Detect package manager and install dependencies
if command -v bun &> /dev/null; then
    echo "Using Bun to update dependencies..."
    bun update
elif command -v yarn &> /dev/null; then
    echo "Using Yarn to install dependencies..."
    yarn install
else
    echo "Using npm to install dependencies..."
    npm install
fi

cd ..

# Change to web directory and start the application
cd web

echo -e "${GREEN}Starting web application...${NC}"
echo "The application will be available at: http://localhost:3001"
echo "Press Ctrl+C to stop the server"
echo ""

# Ensure we're in the web directory and show current directory
echo "Current directory: $(pwd)"
echo "Checking for package.json: $(ls -la package.json 2>/dev/null || echo 'package.json not found')"

# Detect package manager and run the dev server
echo "Checking available package managers..."

if command -v bun &> /dev/null; then
    echo "Found Bun: $(which bun)"
    echo "Using Bun to run dev server..."
    bun run dev
elif command -v yarn &> /dev/null; then
    echo "Found Yarn: $(which yarn)"
    echo "Using Yarn to run dev server..."
    yarn dev
elif command -v npm &> /dev/null; then
    echo "Found npm: $(which npm)"
    echo "Using npm to run dev server..."
    npm run dev
else
    echo "Error: No package manager (bun, yarn, or npm) found in PATH"
    echo "Please install one of: bun, yarn, or npm"
    exit 1
fi
