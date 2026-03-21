#!/bin/bash
# Creates a macOS .app bundle for Gosh GitHub Backup Manager
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
APP_NAME="Gosh GitHub Backup Manager"
BUNDLE_ID="com.goshitsarch.github-backup-manager"
BINARY_NAME="gosh-github-backup-manager"
VERSION="2.0.0"

# Build release binary
echo "Building release binary..."
cd "$PROJECT_DIR"
cargo build --release

# Create .app bundle structure
APP_DIR="$PROJECT_DIR/target/release/${APP_NAME}.app"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Copy binary
cp "$PROJECT_DIR/target/release/$BINARY_NAME" "$APP_DIR/Contents/MacOS/$BINARY_NAME"

# Copy icon
cp "$PROJECT_DIR/assets/icon.icns" "$APP_DIR/Contents/Resources/AppIcon.icns"

# Create Info.plist
cat > "$APP_DIR/Contents/Info.plist" << PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleExecutable</key>
    <string>${BINARY_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
</dict>
</plist>
PLIST

echo ""
echo "App bundle created at:"
echo "  $APP_DIR"
echo ""
echo "To run:  open \"$APP_DIR\""
echo "To install: cp -r \"$APP_DIR\" /Applications/"
