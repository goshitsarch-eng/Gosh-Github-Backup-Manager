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

# Check if .deb file exists
DEB_FILE="$PACKAGING_DIR/gosh-github-backup-manager_1.0.0_amd64.deb"
if [ ! -f "$DEB_FILE" ]; then
    echo "Error: .deb file not found at $DEB_FILE"
    echo "Please build the application first with: npm run build"
    echo "The .deb file will be in src-tauri/target/release/bundle/deb/"

    # Try to find and copy the .deb file
    BUILT_DEB=$(find "$PROJECT_ROOT/src-tauri/target/release/bundle/deb" -name "*.deb" 2>/dev/null | head -1)
    if [ -n "$BUILT_DEB" ]; then
        echo "Found .deb file: $BUILT_DEB"
        cp "$BUILT_DEB" "$DEB_FILE"
        echo "Copied to $DEB_FILE"
    else
        exit 1
    fi
fi

# Build the Flatpak
cd "$SCRIPT_DIR"
echo "Building Flatpak..."
flatpak-builder --force-clean --user --install-deps-from=flathub \
    --repo=repo \
    build-dir \
    com.goshitsarch-eng.gosh-github-backup-manager.yaml

# Create the bundle
echo "Creating Flatpak bundle..."
flatpak build-bundle repo \
    "$PACKAGING_DIR/gosh-github-backup-manager.flatpak" \
    com.goshitsarch-eng.gosh-github-backup-manager

echo ""
echo "Build complete!"
echo "Flatpak bundle: $PACKAGING_DIR/gosh-github-backup-manager.flatpak"
echo ""
echo "To install locally: flatpak install --user $PACKAGING_DIR/gosh-github-backup-manager.flatpak"
echo "To run: flatpak run com.goshitsarch-eng.gosh-github-backup-manager"
