# Tauri macOS Signing & DMG Guide

## Overview

Tauri has built-in support for code signing and notarization. Most configuration
is handled via environment variables and `src-tauri/tauri.conf.json`.

## Prerequisites

- macOS with Xcode Command Line Tools
- Apple Developer account
- Developer ID Application certificate installed in Keychain
- App-specific password for notarization

## Credentials (do not commit)

Set these as environment variables (or in a local `.env` file that is ignored by git):

```bash
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)"
export APPLE_ID="you@example.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="TEAMID"
```

## Step 1: Find your signing identity

Run this to find your Developer ID certificate:

```bash
security find-identity -v -p codesigning
```

Look for a line like:

```
"Developer ID Application: Your Name (TEAMID)"
```

The TEAMID is the 10-character code in parentheses.

## Step 2: Verify `tauri.conf.json`

Confirm the bundle section is set for DMG/app builds and the correct identifier:

```json
{
  "bundle": {
    "active": true,
    "targets": "all",
    "identifier": "com.goshitsarch-eng.gosh-github-backup-manager",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "macOS": {
      "minimumSystemVersion": "10.15"
    }
  }
}
```

> **Note:** Use `"targets": "all"` for all platforms, or `["dmg", "app"]` for macOS-only builds.

## Step 3: Build with signing

With the environment variables set, Tauri handles signing and notarization:

```bash
npm run build
# or
cargo tauri build
```

## Manual signing (if needed)

If automatic signing fails, fall back to a manual signing script. Keep it local
or add it under `scripts/` and make sure secrets are not committed.

## Troubleshooting

### "No identity found"
- Ensure the certificate is installed in Keychain.
- Run `security find-identity -v -p codesigning`.
- Check the certificate has not expired.

### Notarization fails
- Verify the app-specific password is correct.
- Check the Team ID matches the certificate.
- Review Apple's notarization log URL provided in the error.

### App crashes on launch

#### Dynamic library issues

If the app crashes with errors like `Library not loaded: /opt/homebrew/.../libssl.3.dylib`,
the app is dynamically linking to system libraries that won't exist on other Macs.

**Fix:** Ensure static linking in `src-tauri/Cargo.toml`:

```toml
# Use native-tls (macOS Security.framework) instead of OpenSSL
reqwest = { version = "0.12", default-features = false, features = ["json", "native-tls"] }

# Statically link OpenSSL for git2
git2 = { version = "0.19", features = ["vendored-openssl"] }
```

After changing dependencies, rebuild with `cargo clean` first:

```bash
cd src-tauri && cargo clean && cd ..
npm run build
```

#### Entitlements issues

Most Tauri apps do not need custom entitlements. If yours does:

Create `src-tauri/entitlements.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.cs.allow-jit</key>
    <true/>
</dict>
</plist>
```

Then reference it in `tauri.conf.json`:

```json
"macOS": {
  "entitlements": "entitlements.plist"
}
```

## Post-build verification

After building, verify the app before distribution:

```bash
# Check for dynamic library dependencies (should show no Homebrew paths)
otool -L "src-tauri/target/release/bundle/macos/Gosh Github Backup Manager.app/Contents/MacOS/Gosh Github Backup Manager"

# Verify code signature
codesign -dv --verbose=4 "src-tauri/target/release/bundle/macos/Gosh Github Backup Manager.app"

# Test the app launches
open "src-tauri/target/release/bundle/macos/Gosh Github Backup Manager.app"
```

**Expected:** No `/opt/homebrew/` or `/usr/local/` paths in `otool` output. Only system frameworks
like `/System/Library/` and `/usr/lib/` should appear.

## Production checklist

- [ ] Certificate installed and valid
- [ ] Team ID identified
- [ ] App-specific password created
- [ ] Environment variables set
- [ ] Bundle identifier set to `com.goshitsarch-eng.gosh-github-backup-manager`
- [ ] Icons in correct formats
- [ ] Build succeeds locally
- [ ] Notarization completes
- [ ] App runs on a clean Mac
