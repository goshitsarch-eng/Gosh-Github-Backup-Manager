#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PACKAGING_DIR="$PROJECT_ROOT/packaging"

echo "Building Gosh Github Backup Manager Flatpak..."

# Check for required tools
if ! command -v flatpak-builder &> /dev/null; then
    echo "Error: flatpak-builder is not installed."
    echo "Install it with: sudo apt install flatpak-builder (Debian/Ubuntu)"
    echo "                 sudo dnf install flatpak-builder (Fedora)"
    exit 1
fi

# Install runtime and SDK if not present
echo "Ensuring GNOME 47 runtime and SDK are installed..."
flatpak install --user -y flathub org.gnome.Platform//47 org.gnome.Sdk//47 || true

# Check if binary exists
BINARY="$PROJECT_ROOT/target/release/gosh-github-backup-manager"
if [ ! -f "$BINARY" ]; then
    echo "Error: Release binary not found at $BINARY"
    echo "Please build the application first with: cargo build --release"

    exit 1
fi

# Copy binary for Flatpak build
mkdir -p "$SCRIPT_DIR/bin"
cp "$BINARY" "$SCRIPT_DIR/bin/"

# Build the Flatpak
cd "$SCRIPT_DIR"
echo "Building Flatpak..."
flatpak-builder --force-clean --user --install-deps-from=flathub \
    --repo=repo \
    build-dir \
    com.goshitsarcheng.gosh-github-backup-manager.yaml

# Create the bundle
echo "Creating Flatpak bundle..."
flatpak build-bundle repo \
    "$PACKAGING_DIR/gosh-github-backup-manager.flatpak" \
    com.goshitsarcheng.gosh-github-backup-manager

# Cleanup
rm -rf "$SCRIPT_DIR/bin"

echo ""
echo "Build complete!"
echo "Flatpak bundle: $PACKAGING_DIR/gosh-github-backup-manager.flatpak"
echo ""
echo "To install locally: flatpak install --user $PACKAGING_DIR/gosh-github-backup-manager.flatpak"
echo "To run: flatpak run com.goshitsarcheng.gosh-github-backup-manager"
