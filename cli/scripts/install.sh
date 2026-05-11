#!/bin/bash
set -e

REPO="shawal-mbalire/shastack"
BINARY="sha"
INSTALL_DIR=""

echo "Installing shastack (sha) CLI..."

# --- Detect OS and architecture ---
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)
    case "$ARCH" in
      x86_64) ASSET="sha-linux-x86_64" ;;
      *) echo "Unsupported architecture: $ARCH. Build from source: https://github.com/${REPO}" && exit 1 ;;
    esac
    ;;
  darwin)
    case "$ARCH" in
      x86_64) ASSET="sha-macos-x86_64" ;;
      arm64)  ASSET="sha-macos-aarch64" ;;
      *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS"
    echo "Windows: powershell -c \"irm https://raw.githubusercontent.com/${REPO}/main/cli/scripts/install.ps1 | iex\""
    exit 1
    ;;
esac

# --- Resolve download URL ---
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"

echo "Downloading ${ASSET}..."
TMP=$(mktemp)
curl -sSfL "$DOWNLOAD_URL" -o "$TMP"
chmod +x "$TMP"

# --- Choose install location ---
if [ -w /usr/local/bin ]; then
  INSTALL_DIR="/usr/local/bin"
else
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
fi

mv "$TMP" "${INSTALL_DIR}/${BINARY}"

echo ""
echo "shastack installed: ${INSTALL_DIR}/${BINARY}"

if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
  echo ""
  echo "Add to your PATH (add this to ~/.bashrc or ~/.zshrc):"
  echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
fi

echo ""
echo "Run 'sha --help' to get started."
