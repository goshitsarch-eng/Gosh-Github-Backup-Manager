# Gosh Github Backup Manager

> Gosh apps are built with a Linux-first mindset: simplicity, transparency, and user control.
>
> We also provide Windows and macOS builds — not as a compromise, but as an on-ramp. Many people are curious about Linux but still live on other platforms day-to-day. If these tools help someone get comfortable and eventually make the jump, we're happy to meet them where they are.

A cross-platform Tauri desktop application to backup your GitHub repositories. Connect your GitHub account, view your repositories, and backup them to your local machine with optional zip archiving.

## Features

- **GitHub Integration**: Connect via Personal Access Token to access all your repositories
- **Dashboard**: View your profile, stats (stars/forks), and recent activity at a glance
- **Repository Browser**: Filter, sort, and select repositories for backup
- **Backup Options**: Choose clone type (full/mirror), organize by owner, set concurrent limits
- **Progress Tracking**: Real-time progress for backup operations via event system
- **Archive Support**: Create zip archives of your backups with configurable compression (0-9)
- **Backup History**: Track your past backup operations (last 50 entries retained)
- **Dark/Light Theme**: Choose light, dark, or system preference

## Installation

### Linux

We provide multiple package formats for Linux. Choose the one that works best for your distribution.

#### AppImage (Universal)

Works on any Linux distribution. No installation required.

```bash
# Download the AppImage
chmod +x "Gosh Github Backup Manager_1.0.0_amd64.AppImage"

# Run it
./"Gosh Github Backup Manager_1.0.0_amd64.AppImage"
```

#### DEB (Debian, Ubuntu, Linux Mint, Pop!_OS)

```bash
sudo dpkg -i "Gosh Github Backup Manager_1.0.0_amd64.deb"

# If you encounter dependency issues:
sudo apt-get install -f
```

**Runtime dependencies:** `git`, `libwebkit2gtk-4.1-0`, `libgtk-3-0`, `libssl3`

#### RPM (Fedora, RHEL, CentOS, openSUSE)

```bash
# Fedora/RHEL
sudo rpm -i "Gosh Github Backup Manager-1.0.0-1.x86_64.rpm"

# Or with dnf
sudo dnf install "Gosh Github Backup Manager-1.0.0-1.x86_64.rpm"
```

**Runtime dependencies:** `git`, `webkit2gtk4.1`, `gtk3`, `openssl-libs`

#### Flatpak

```bash
# Install from bundle
flatpak install --user gosh-github-backup-manager.flatpak

# Run
flatpak run com.goshitsarcheng.gosh-github-backup-manager
```

The Flatpak uses GNOME 47 runtime (based on Freedesktop SDK 24.08) and includes all dependencies.

### Windows

Download the `.msi` or `.exe` installer from the releases page and run it.

### macOS

Download the `.dmg` file from the releases page, open it, and drag the app to your Applications folder.

## Building from Source

### Prerequisites

- Node.js 18+
- Rust (latest stable)
- npm or yarn
- Git

#### Linux Build Dependencies

```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libssl-dev \
  librsvg2-dev libayatana-appindicator3-dev build-essential \
  curl wget file libxdo-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel gtk3-devel openssl-devel \
  librsvg2-devel libappindicator-gtk3-devel curl wget file \
  libxdo-devel
sudo dnf group install "C Development Tools and Libraries"

# Arch Linux
sudo pacman -S webkit2gtk-4.1 gtk3 openssl librsvg \
  libappindicator-gtk3 base-devel curl wget file xdotool
```

### Build Steps

```bash
# Clone the repository
git clone https://github.com/Gosh-Its-Arch/Github-Backup-Manager.git
cd Github-Backup-Manager

# Install dependencies
npm install

# Start development server
npm run dev

# Build for production (creates installers in src-tauri/target/release/bundle/)
npm run build
```

### Building All Linux Packages

```bash
# Build AppImage, DEB, RPM, and Flatpak
./packaging/build-all.sh

# Or build specific formats
./packaging/build-all.sh --appimage
./packaging/build-all.sh --deb
./packaging/build-all.sh --rpm
./packaging/build-all.sh --flatpak
```

## Tech Stack

**Frontend:**
- React 18 - UI framework
- TypeScript 5.3 - Type safety
- Vite 5.0 - Build tool
- Tailwind CSS 3.4 - Styling
- shadcn/ui + Radix UI - Component library

**Backend (Rust):**
- Tauri 2 - Cross-platform desktop framework
- Tokio - Async runtime
- git2 0.19 - Git operations (with vendored OpenSSL)
- reqwest 0.12 - HTTP client (native-tls)
- zip 2 - Archive creation
- serde/serde_json - Serialization
- chrono 0.4 - Date/time handling

**Tauri Plugins:**
- tauri-plugin-shell - Shell command execution
- tauri-plugin-dialog - Native file dialogs
- tauri-plugin-os - OS information
- tauri-plugin-single-instance - Prevent multiple app instances (desktop only)

## Project Structure

```
src/
├── renderer/            # React frontend
│   ├── App.tsx          # Main app with routing
│   ├── components/      # UI components (Auth, Dashboard, Repositories, Backup, Settings, Layout)
│   ├── context/         # React context (AppContext)
│   ├── hooks/           # Custom hooks (useGitHub, useBackup, useSettings)
│   ├── lib/             # Utilities & Tauri API bridge
│   └── styles/          # Global styles
└── shared/              # Shared TypeScript types
    ├── types.ts         # Interface definitions
    └── constants.ts     # App constants & defaults

src-tauri/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Main Tauri app & IPC commands
│   ├── github.rs        # GitHub API service
│   ├── git.rs           # Git operations service
│   ├── archive.rs       # Archive/zip service
│   ├── storage.rs       # Persistent JSON storage service
│   └── types.rs         # Rust type definitions
├── Cargo.toml           # Rust dependencies
└── tauri.conf.json      # Tauri configuration

packaging/
├── build-all.sh         # Master build script
├── *.desktop            # Desktop entry files
├── *.metainfo.xml       # AppStream metadata
├── flatpak/             # Flatpak manifest and build files
├── deb/                 # Debian packaging files
└── rpm/                 # RPM spec file
```

## Configuration

### GitHub Token

The app requires a GitHub Personal Access Token with the following scopes:
- `repo` - Access to private repositories
- `read:user` - Read user profile
- `read:org` - Read organization membership

Create a token at: https://github.com/settings/tokens/new

### Data Storage

Application data is stored in a JSON file at your system's app data directory:
- **Linux**: `~/.local/share/com.goshitsarch-eng.gosh-github-backup-manager/gosh-github-backup-manager.json`
- **macOS**: `~/Library/Application Support/com.goshitsarch-eng.gosh-github-backup-manager/gosh-github-backup-manager.json`
- **Windows**: `%APPDATA%\com.goshitsarch-eng.gosh-github-backup-manager\gosh-github-backup-manager.json`

This file contains your GitHub token, settings, and backup history. On Unix systems, file permissions are set to 0600 (owner read/write only).

## Clone Types

| Type | Description |
|------|-------------|
| `full` | Standard git clone with full history and working directory |
| `mirror` | Bare repository clone (no working directory, refs only) |

**Note:** If a repository already exists at the destination, the app will perform a pull/fetch operation instead of cloning.

## Scripts

| Script | Description |
|--------|-------------|
| `npm run dev` | Start Tauri development server |
| `npm run dev:vite` | Start Vite dev server only |
| `npm run build` | Build for production |
| `npm run build:vite` | Build frontend only |
| `npm run typecheck` | Run TypeScript type checking |
| `npm run lint` | Run ESLint |

## Known Limitations

1. **Shallow clone**: Not currently supported by the git2 library; selecting "shallow" performs a full clone
2. **Auto-backup scheduling**: Settings exist in the UI but scheduled backups are not implemented
3. **Token storage**: Stored in plain text JSON file (not in secure keychain)

## License

AGPL-3.0
