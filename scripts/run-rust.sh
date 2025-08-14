#!/bin/bash

# Vehicle Net - Rust Build and Run Script
# This script builds and runs the Rust application from the root directory

set -e

echo "🚗 Building Vehicle Net Rust Application..."
echo "=========================================="

# Change to rust directory
cd rust

# Build the application
echo "📦 Building..."
cargo build --release

# Run the application with default arguments
echo ""
echo "🏃 Running Vehicle Net..."
echo "========================="

# Check if arguments were passed
if [ $# -eq 0 ]; then
    # No arguments, run with default index 0
    echo "No arguments provided, using default vehicle index 0"
    echo "Usage: ./run-rust.sh [--index <number>]"
    echo ""
    cargo run --release -- --index 0
else
    # Pass all arguments to cargo run
    cargo run --release -- "$@"
fi

echo ""
echo "✅ Done!"