# Gosh Github Backup Manager

> Gosh apps are built with a Linux-first mindset: simplicity, transparency, and user control.
>
> We also provide Windows and macOS builds not as a compromise, but as an on-ramp. Many people are curious about Linux but still live on other platforms day-to-day. If these tools help someone get comfortable and eventually make the jump, we're happy to meet them where they are.

A cross-platform native desktop application to backup your GitHub repositories. Built with Rust and the iced GUI framework -- no web runtime, no JavaScript, just a fast native binary.

## Features

- **GitHub Integration**: Connect via Personal Access Token to access all your repositories
- **Dashboard**: View your profile, stats (stars/forks), and recent activity at a glance
- **Repository Browser**: Filter, sort, and select repositories for backup
- **Backup Options**: Choose clone type (full/mirror), organize by owner, set concurrent limits
- **Progress Tracking**: Real-time progress for backup operations
- **Archive Support**: Create zip archives of your backups with configurable compression (0-9)
- **Backup History**: Track your past backup operations (last 50 entries retained)
- **Dark/Light Theme**: Choose light, dark, or system preference

## Screenshots

![Screenshot 1](screenshots/img1.png)
![Screenshot 2](screenshots/img2.png)
![Screenshot 3](screenshots/img3.png)
![Screenshot 4](screenshots/img4.png)


## Installation

### Linux

We provide multiple package formats for Linux. Choose the one that works best for your distribution.

#### Binary

Download the binary from the releases page:

```bash
chmod +x gosh-github-backup-manager
./gosh-github-backup-manager
```

#### Flatpak

```bash
# Install from bundle
flatpak install --user gosh-github-backup-manager.flatpak

# Run
flatpak run com.goshitsarcheng.gosh-github-backup-manager
```

### Windows

Download the `.exe` from the releases page and run it.

### macOS

Download the binary from the releases page and run it.

## Building from Source

### Prerequisites

- Rust (latest stable)
- Git

### Build Steps

```bash
# Clone the repository
git clone https://github.com/Gosh-Its-Arch/Github-Backup-Manager.git
cd Github-Backup-Manager

# Build for release
cargo build --release

# Run
./target/release/gosh-github-backup-manager
```

### Building All Linux Packages

```bash
# Build binary and Flatpak
./packaging/build-all.sh

# Or build specific formats
./packaging/build-all.sh --binary
./packaging/build-all.sh --flatpak
```

## Tech Stack

- **[iced](https://github.com/iced-rs/iced)** - Cross-platform GUI framework for Rust
- **Tokio** - Async runtime
- **git2 0.19** - Git operations (with vendored OpenSSL)
- **reqwest 0.12** - HTTP client (native-tls)
- **zip 2** - Archive creation
- **serde/serde_json** - Serialization
- **chrono 0.4** - Date/time handling
- **rfd** - Native file dialogs

## Project Structure

```
src/
├── main.rs              # Entry point, iced application setup
├── app.rs               # Main application state and update logic
├── theme.rs             # Theme definitions (light/dark)
├── types.rs             # Shared type definitions
├── pages/               # UI pages
│   ├── auth.rs          # Authentication screen
│   ├── dashboard.rs     # Dashboard with stats and activity
│   ├── repositories.rs  # Repository browser with filters
│   ├── backup.rs        # Backup options, progress, and history
│   ├── settings.rs      # App settings
│   └── about.rs         # About page
├── widgets/             # Reusable UI widgets
│   ├── sidebar.rs       # Navigation sidebar
│   └── repo_card.rs     # Repository card component
└── services/            # Backend services
    ├── github.rs        # GitHub API client
    ├── git.rs           # Git clone/pull operations
    ├── archive.rs       # ZIP archive creation
    └── storage.rs       # Persistent JSON storage

assets/                  # Application icons
packaging/               # Linux packaging files (Flatpak, DEB, RPM)
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
- **Linux**: `~/.local/share/gosh-github-backup-manager/gosh-github-backup-manager.json`
- **macOS**: `~/Library/Application Support/gosh-github-backup-manager/gosh-github-backup-manager.json`
- **Windows**: `%APPDATA%\gosh-github-backup-manager\gosh-github-backup-manager.json`

This file contains your GitHub token, settings, and backup history. On Unix systems, file permissions are set to 0600 (owner read/write only).

## Clone Types

| Type | Description |
|------|-------------|
| `full` | Standard git clone with full history and working directory |
| `mirror` | Bare repository clone (no working directory, refs only) |

**Note:** If a repository already exists at the destination, the app will perform a pull/fetch operation instead of cloning.

## Known Limitations

1. **Shallow clone**: Not currently supported by the git2 library; selecting "shallow" performs a full clone
2. **Auto-backup scheduling**: Settings exist in the UI but scheduled backups are not implemented
3. **Token storage**: Stored in plain text JSON file (not in secure keychain)

### Disclaimer

This application is an independent project and is not sponsored by, endorsed by, or affiliated with GitHub or GitHub, Inc.

This software is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0).
It is provided **"as is"**, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability or fitness for a particular purpose.
Use at your own risk.


## License

AGPL-3.0
