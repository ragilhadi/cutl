#!/bin/bash
set -e

# cutl CLI installer script - GitHub Release version
# Downloads and installs the latest cutl CLI from GitHub releases

REPO="ragilhadi/cutl"  # Change this to your GitHub username/repo
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="cutl"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        OS_TYPE="linux"
        ;;
    Darwin*)
        OS_TYPE="darwin"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        OS_TYPE="windows"
        BINARY_NAME="cutl.exe"
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        ARCH_TYPE="x86_64"
        ;;
    aarch64|arm64)
        ARCH_TYPE="aarch64"
        ;;
    armv7l)
        ARCH_TYPE="armv7"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

PLATFORM="${OS_TYPE}-${ARCH_TYPE}"

echo "Installing cutl CLI for $PLATFORM..."
echo "Install directory: $INSTALL_DIR"

# Get the latest release version
echo "Fetching latest release..."
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_RELEASE" ]; then
    echo "Error: Could not fetch latest release information"
    echo "Please check if the repository has any releases published"
    exit 1
fi

echo "Latest version: $LATEST_RELEASE"

# Construct download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/cutl-${PLATFORM}.tar.gz"

echo "Downloading from: $DOWNLOAD_URL"

# Create temporary directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download the release
if ! curl -L -o "$TMP_DIR/cutl.tar.gz" "$DOWNLOAD_URL"; then
    echo "Error: Failed to download release"
    echo "URL: $DOWNLOAD_URL"
    exit 1
fi

# Extract the binary
echo "Extracting..."
tar -xzf "$TMP_DIR/cutl.tar.gz" -C "$TMP_DIR"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Install the binary
cp "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "✓ Installed to $INSTALL_DIR/$BINARY_NAME"

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
echo "Version: $LATEST_RELEASE"
