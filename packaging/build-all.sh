#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=========================================="
echo "Gosh Github Backup Manager - Build Script"
echo "=========================================="
echo ""

# Parse arguments
BUILD_ALL=false
BUILD_APPIMAGE=false
BUILD_DEB=false
BUILD_RPM=false
BUILD_FLATPAK=false

if [ $# -eq 0 ]; then
    BUILD_ALL=true
fi

for arg in "$@"; do
    case $arg in
        --all)
            BUILD_ALL=true
            ;;
        --appimage)
            BUILD_APPIMAGE=true
            ;;
        --deb)
            BUILD_DEB=true
            ;;
        --rpm)
            BUILD_RPM=true
            ;;
        --flatpak)
            BUILD_FLATPAK=true
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --all       Build all package formats (default)"
            echo "  --appimage  Build AppImage only"
            echo "  --deb       Build DEB package only"
            echo "  --rpm       Build RPM package only"
            echo "  --flatpak   Build Flatpak only"
            echo "  --help      Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $arg"
            exit 1
            ;;
    esac
done

if [ "$BUILD_ALL" = true ]; then
    BUILD_APPIMAGE=true
    BUILD_DEB=true
    BUILD_RPM=true
    BUILD_FLATPAK=true
fi

# Check for required tools
echo "Checking build dependencies..."

if ! command -v npm &> /dev/null; then
    echo "Error: npm is not installed"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "Error: cargo (Rust) is not installed"
    exit 1
fi

cd "$PROJECT_ROOT"

# Build the Tauri application (generates AppImage, DEB, RPM)
echo ""
echo "Building Tauri application..."
echo ""

npm ci
npm run build

# Output locations
BUNDLE_DIR="$PROJECT_ROOT/src-tauri/target/release/bundle"

echo ""
echo "=========================================="
echo "Build Results"
echo "=========================================="

if [ "$BUILD_APPIMAGE" = true ] && [ -d "$BUNDLE_DIR/appimage" ]; then
    echo ""
    echo "AppImage:"
    ls -la "$BUNDLE_DIR/appimage/"*.AppImage 2>/dev/null || echo "  Not found"
fi

if [ "$BUILD_DEB" = true ] && [ -d "$BUNDLE_DIR/deb" ]; then
    echo ""
    echo "DEB Package:"
    ls -la "$BUNDLE_DIR/deb/"*.deb 2>/dev/null || echo "  Not found"
fi

if [ "$BUILD_RPM" = true ] && [ -d "$BUNDLE_DIR/rpm" ]; then
    echo ""
    echo "RPM Package:"
    ls -la "$BUNDLE_DIR/rpm/"*.rpm 2>/dev/null || echo "  Not found"
fi

# Build Flatpak if requested
if [ "$BUILD_FLATPAK" = true ]; then
    echo ""
    echo "Building Flatpak..."

    if ! command -v flatpak-builder &> /dev/null; then
        echo "Warning: flatpak-builder is not installed, skipping Flatpak build"
        echo "Install it with: sudo apt install flatpak-builder (Debian/Ubuntu)"
        echo "                 sudo dnf install flatpak-builder (Fedora)"
    else
        # Copy the deb file to packaging directory
        DEB_FILE=$(find "$BUNDLE_DIR/deb" -name "*.deb" | head -1)
        if [ -n "$DEB_FILE" ]; then
            cp "$DEB_FILE" "$SCRIPT_DIR/gosh-github-backup-manager_1.0.0_amd64.deb"

            cd "$SCRIPT_DIR/flatpak"
            chmod +x build-flatpak.sh
            ./build-flatpak.sh

            echo ""
            echo "Flatpak:"
            ls -la "$SCRIPT_DIR/"*.flatpak 2>/dev/null || echo "  Not found"
        else
            echo "Warning: DEB file not found, cannot build Flatpak"
        fi
    fi
fi

echo ""
echo "=========================================="
echo "Build complete!"
echo "=========================================="
