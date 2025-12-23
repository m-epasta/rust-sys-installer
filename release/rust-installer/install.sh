#!/bin/bash

# Rust Development Environment Installer
# Auto-executing script that handles its own permissions

# Check if we're executable - if not, make ourselves executable and re-run
if [ ! -x "$0" ]; then
    echo "🔧 Making installer executable..."
    chmod +x "$0"
    echo "✅ Permissions fixed! Re-executing installer..."
    # Use full path for re-execution
    SCRIPT_PATH="$(cd "$(dirname "$0")" && pwd)/$(basename "$0")"
    exec "$SCRIPT_PATH" "$@"
fi

# Now we're guaranteed to be executable - proceed with installation
set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${GREEN}🚀 Rust Development Environment Installer${NC}"
echo -e "${BLUE}========================================${NC}"

# Update apt-get for accessing latest versions of the packages
sudo apt-get update
echo -e "${GREEN}apt-get succesfully updated${NC}"

echo -e "${GREEN}INSTALLATION READY TO PROCEED${NC}"

# Check if binary exists
BINARY_NAME="rust-sys-installer"
BINARY_PATH="${SCRIPT_DIR}/${BINARY_NAME}"

if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}❌ Error: Rust binary not found at ${BINARY_PATH}${NC}"
    echo -e "${YELLOW}Please make sure to build the project first with: cargo build --release${NC}"
    exit 1
fi

echo -e "${YELLOW}📦 Preparing installer...${NC}"

# Generate unique name for temp binary to avoid conflicts
TEMP_BINARY="/tmp/${BINARY_NAME}_$$_$(date +%s)"

# Cleanup function
cleanup() {
    if [ -f "$TEMP_BINARY" ]; then
        rm -f "$TEMP_BINARY"
        echo -e "${YELLOW}🧹 Cleaned up temporary binary${NC}"
    fi
}

# Set trap to cleanup on exit
trap cleanup EXIT

# Copy binary to /tmp
if cp "$BINARY_PATH" "$TEMP_BINARY"; then
    echo -e "${GREEN}✅ Binary copied to ${TEMP_BINARY}${NC}"
else
    echo -e "${RED}❌ Failed to copy binary to /tmp${NC}"
    exit 1
fi

# Ensure it's executable in /tmp
if chmod +x "$TEMP_BINARY"; then
    echo -e "${GREEN}✅ Binary is executable${NC}"
else
    echo -e "${RED}❌ Failed to make binary executable${NC}"
    exit 1
fi

echo -e "${BLUE}🔧 Starting installation...${NC}"
echo -e "${YELLOW}Note: This installer requires sudo access for system package installation${NC}"
echo ""

# Execute the binary from /tmp
if "$TEMP_BINARY"; then
    echo ""
    echo -e "${GREEN}🎉 Installation completed successfully!${NC}"
else
    EXIT_CODE=$?
    echo ""
    echo -e "${RED}❌ Installation failed with exit code: ${EXIT_CODE}${NC}"
    exit $EXIT_CODE
fi

# Clean up (trap will handle this automatically)
echo -e "${YELLOW}🧹 Cleaning up temporary files...${NC}"
echo -e "${GREEN}✅ Cleanup completed${NC}"

echo ""
echo -e "${GREEN}🎊 Your Rust development environment is now ready!${NC}"
echo -e "${BLUE}Happy coding! 🚀${NC}"
