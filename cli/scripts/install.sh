#!/bin/bash
set -e

echo "Installing shastack (sha) CLI..."

if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is not installed. Please install it from https://rustup.rs/"
    exit 1
fi

echo "Running cargo install --git https://github.com/shawal-mbalire/shastack..."
cargo install --git https://github.com/shawal-mbalire/shastack

echo "shastack (sha) has been installed successfully!"
echo "Make sure ~/.cargo/bin is in your PATH."
