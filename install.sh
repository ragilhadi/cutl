#!/bin/bash
set -e

# cutl CLI installer script
# Installs the cutl CLI tool to your system

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Installing cutl CLI..."
echo "Install directory: $INSTALL_DIR"

# Build the CLI if not already built
if [ ! -f "$PROJECT_ROOT/target/release/cutl" ]; then
    echo "Building cutl CLI..."
    cd "$PROJECT_ROOT"
    cargo build --release -p cutl
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy the binary
cp "$PROJECT_ROOT/target/release/cutl" "$INSTALL_DIR/cutl"
chmod +x "$INSTALL_DIR/cutl"

echo "✓ Installed to $INSTALL_DIR/cutl"

# Check if INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  WARNING: $INSTALL_DIR is not in your PATH"
    echo ""
    echo "Add this to your ~/.bashrc or ~/.zshrc:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
    echo "Then run: source ~/.bashrc  (or source ~/.zshrc)"
fi

echo ""
echo "Installation complete! Run: cutl --help"
