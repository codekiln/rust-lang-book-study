#!/bin/bash
set -e

echo "🦀 Rust Development Environment Setup"
echo "======================================"

# Display Rust version
echo "📦 Rust version:"
rustc --version
cargo --version

# Install rustfmt and clippy for the active toolchain
echo ""
echo "🔧 Installing rustfmt and clippy for active toolchain..."
rustup component add rustfmt clippy

echo ""
echo "✅ Setup complete!"
