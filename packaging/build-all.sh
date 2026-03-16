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
BUILD_BINARY=false
BUILD_FLATPAK=false

if [ $# -eq 0 ]; then
    BUILD_ALL=true
fi

for arg in "$@"; do
    case $arg in
        --all)
            BUILD_ALL=true
            ;;
        --binary)
            BUILD_BINARY=true
            ;;
        --flatpak)
            BUILD_FLATPAK=true
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo ""
            echo "Options:"
            echo "  --all       Build binary and Flatpak (default)"
            echo "  --binary    Build release binary only"
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
    BUILD_BINARY=true
    BUILD_FLATPAK=true
fi

# Check for required tools
echo "Checking build dependencies..."

if ! command -v cargo &> /dev/null; then
    echo "Error: cargo (Rust) is not installed"
    exit 1
fi

cd "$PROJECT_ROOT"

# Build the release binary
if [ "$BUILD_BINARY" = true ]; then
    echo ""
    echo "Building release binary..."
    echo ""

    cargo build --release

    echo ""
    echo "Binary:"
    ls -la "$PROJECT_ROOT/target/release/gosh-github-backup-manager" 2>/dev/null || echo "  Not found"
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
        # Ensure binary is built
        if [ ! -f "$PROJECT_ROOT/target/release/gosh-github-backup-manager" ]; then
            echo "Building release binary first..."
            cargo build --release
        fi

        mkdir -p "$SCRIPT_DIR/flatpak/bin"
        cp "$PROJECT_ROOT/target/release/gosh-github-backup-manager" "$SCRIPT_DIR/flatpak/bin/"

        cd "$SCRIPT_DIR/flatpak"
        chmod +x build-flatpak.sh
        ./build-flatpak.sh

        echo ""
        echo "Flatpak:"
        ls -la "$SCRIPT_DIR/"*.flatpak 2>/dev/null || echo "  Not found"
    fi
fi

echo ""
echo "=========================================="
echo "Build complete!"
echo "=========================================="
