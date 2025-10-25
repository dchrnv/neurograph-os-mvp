#!/bin/bash

# Token V2.0 Rust - Setup and Test Script
# Version: 0.12.0 mvp_TokenR

set -e  # Exit on error

BOLD="\033[1m"
GREEN="\033[0;32m"
YELLOW="\033[0;33m"
RED="\033[0;31m"
NC="\033[0m" # No Color

echo -e "${BOLD}=== NeuroGraph OS Token V2.0 Rust - Setup & Test ===${NC}\n"

# Function to print status
print_status() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Check if Rust is installed
echo -e "${BOLD}Step 1: Checking Rust installation${NC}"
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    print_status "Cargo found: $CARGO_VERSION"
else
    print_error "Cargo not found!"
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "After installation, restart your terminal and run this script again."
    exit 1
fi

if command -v rustc &> /dev/null; then
    RUSTC_VERSION=$(rustc --version)
    print_status "Rustc found: $RUSTC_VERSION"
else
    print_error "Rustc not found!"
    exit 1
fi

echo ""

# Build project
echo -e "${BOLD}Step 2: Building project${NC}"
print_status "Running cargo build..."
cargo build
print_status "Build completed successfully"
echo ""

# Run tests
echo -e "${BOLD}Step 3: Running tests${NC}"
print_status "Running cargo test..."
cargo test
echo ""

# Build release
echo -e "${BOLD}Step 4: Building release version${NC}"
print_status "Running cargo build --release..."
cargo build --release
print_status "Release build completed"
echo ""

# Run demo
echo -e "${BOLD}Step 5: Running demo${NC}"
print_status "Running token-demo..."
echo ""
echo "─────────────────────────────────────────────────────────"
cargo run --bin token-demo
echo "─────────────────────────────────────────────────────────"
echo ""

# Summary
echo -e "${BOLD}=== Summary ===${NC}"
print_status "Token V2.0 Rust implementation is working!"
print_status "All tests passed"
print_status "Demo executed successfully"
echo ""

echo -e "${BOLD}Next steps:${NC}"
echo "  • View API docs: cargo doc --open"
echo "  • Run benchmarks: cargo bench (when added)"
echo "  • Format code: cargo fmt"
echo "  • Lint code: cargo clippy"
echo ""

echo -e "${BOLD}Build artifacts:${NC}"
echo "  • Debug binary: target/debug/token-demo"
echo "  • Release binary: target/release/token-demo"
echo "  • Library: target/release/libneurograph_core.rlib"
echo ""

print_status "Setup complete! Token V2.0 is ready for use."
