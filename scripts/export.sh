#!/usr/bin/env bash
# Export using the matching standard (GDScript) Godot templates.
set -euo pipefail
cd "$(dirname "$0")/.."
platform=${1:-Linux}
arch=${2:-x86_64}
case "$platform" in
  Linux) default=target/release/gosh-github-backup-manager ;;
  Windows) default=target/release/gosh-github-backup-manager.exe ;;
  macOS) default="target/release/Gosh GitHub Backup Manager.app" ;;
  *) echo "Unknown platform: $platform" >&2; exit 2 ;;
esac
output=${3:-$default}
mkdir -p "$(dirname "$output")"
absolute="$(cd "$(dirname "$output")" && pwd)/$(basename "$output")"
case "$(godot --version)" in
  4.7.2.stable*) ;;
  *) echo 'Godot 4.7.2 stable is required.' >&2; exit 1 ;;
esac
godot --headless --path desktop --import
godot --headless --path desktop --export-release "$platform $arch" "$absolute"
