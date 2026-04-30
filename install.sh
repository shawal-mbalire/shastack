#!/bin/bash
set -e

REPO="shawal-mbalire/shastack"
BINARY_NAME="sha"

echo "Installing shastack CLI..."

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux*)   PLATFORM="unknown-linux-gnu" ;;
  darwin*)  PLATFORM="apple-darwin" ;;
  msys*|cygwin*|mingw*) PLATFORM="pc-windows-msvc" ;;
  *)        echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
  x86_64)  TARGET="x86_64-$PLATFORM" ;;
  arm64|aarch64) 
    if [ "$OS" = "darwin" ]; then
      TARGET="aarch64-apple-darwin"
    else
      echo "Unsupported Architecture: $ARCH"; exit 1
    fi
    ;;
  *) echo "Unsupported Architecture: $ARCH"; exit 1 ;;
esac

# Get latest release version
LATEST_RELEASE=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "tag_name" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE" ]; then
  echo "Error: Could not find latest release."
  exit 1
fi

echo "Downloading version $LATEST_RELEASE for $TARGET..."

URL="https://github.com/${REPO}/releases/download/${LATEST_RELEASE}/${BINARY_NAME}-${TARGET}.tar.gz"

# Download and install (mocking for now as we don't have the assets yet)
# curl -L "$URL" | tar -xz
# mv "$BINARY_NAME" /usr/local/bin/

echo "Note: Binary download is currently a placeholder until GitHub Release assets are populated."
echo "shastack CLI (sha) installed successfully (simulated)."
