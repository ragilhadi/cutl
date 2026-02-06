#!/bin/bash
set -e

# cutl CLI uninstaller script
# Removes the cutl CLI tool from your system

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo "Uninstalling cutl CLI..."
echo "Install directory: $INSTALL_DIR"

if [ -f "$INSTALL_DIR/cutl" ]; then
    rm "$INSTALL_DIR/cutl"
    echo "✓ Removed $INSTALL_DIR/cutl"
else
    echo "⚠️  cutl not found in $INSTALL_DIR"
fi

echo ""
echo "Uninstall complete!"
