#!/bin/bash

# FRMWRX Harvester - Build and Run Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════╗"
    echo "║          FRMWRX HARVESTER             ║"
    echo "║      Professional Chat Interface     ║"
    echo "╚═══════════════════════════════════════╝"
    echo -e "${NC}"
}

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_dependencies() {
    print_info "Checking dependencies..."
    
    # Check Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust first."
        echo "Visit: https://rustup.rs/"
        exit 1
    fi
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js not found. Please install Node.js first."
        echo "Visit: https://nodejs.org/"
        exit 1
    fi
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        print_error "npm not found. Please install npm first."
        exit 1
    fi
    
    print_success "All dependencies found!"
}

setup() {
    print_info "Setting up FRMWRX Harvester..."
    
    # Install Rust dependencies
    print_info "Installing Rust dependencies..."
    cargo build
    
    # Install frontend dependencies
    print_info "Installing frontend dependencies..."
    cd ui
    npm install
    cd ..
    
    print_success "Setup completed!"
}

dev() {
    print_info "Starting development server..."
    cargo tauri dev
}

build() {
    print_info "Building for production..."
    cargo tauri build
    print_success "Build completed! Check the target/release directory."
}

test() {
    print_info "Running tests..."
    
    # Run Rust tests
    print_info "Running Rust tests..."
    cargo test
    
    # Run frontend tests if they exist
    if [ -f "ui/package.json" ] && grep -q "\"test\"" ui/package.json; then
        print_info "Running frontend tests..."
        cd ui && npm test && cd ..
    fi
    
    print_success "All tests completed!"
}

clean() {
    print_info "Cleaning build artifacts..."
    cargo clean
    rm -rf ui/node_modules
    rm -rf ui/dist
    print_success "Clean completed!"
}

usage() {
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  setup     - Install all dependencies"
    echo "  dev       - Start development server"
    echo "  build     - Build for production"
    echo "  test      - Run all tests"
    echo "  clean     - Clean build artifacts"
    echo "  check     - Check dependencies"
    echo ""
    echo "Examples:"
    echo "  $0 setup    # First time setup"
    echo "  $0 dev      # Start development"
    echo "  $0 build    # Build for release"
}

# Main script
print_banner

case "${1:-}" in
    setup)
        check_dependencies
        setup
        ;;
    dev)
        check_dependencies
        dev
        ;;
    build)
        check_dependencies
        build
        ;;
    test)
        check_dependencies
        test
        ;;
    clean)
        clean
        ;;
    check)
        check_dependencies
        ;;
    *)
        usage
        exit 1
        ;;
esac