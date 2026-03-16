Name:           gosh-github-backup-manager
Version:        2.0.0
Release:        1%{?dist}
Summary:        A cross-platform app to backup your GitHub repositories

License:        AGPL-3.0
URL:            https://github.com/Gosh-Its-Arch/Github-Backup-Manager
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  openssl-devel
BuildRequires:  pkg-config

Requires:       git
Requires:       openssl-libs

%description
Gosh Github Backup Manager is a native desktop application built with Rust
and iced that helps you backup your GitHub repositories to your local machine.
It provides an intuitive interface for managing your repository backups with
support for multiple backup modes including clone and mirror.

Features:
- GitHub authentication via Personal Access Token
- Browse and filter your repositories
- Multiple backup modes: clone and mirror
- Optional ZIP archive creation with configurable compression
- Backup progress tracking
- Backup history management
- Light and dark theme support

%prep
%autosetup

%build
cargo build --release

%install
rm -rf %{buildroot}

# Install binary
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install desktop file
install -Dm644 packaging/com.goshitsarch-eng.gosh-github-backup-manager.desktop \
    %{buildroot}%{_datadir}/applications/com.goshitsarch-eng.gosh-github-backup-manager.desktop

# Install metainfo
install -Dm644 packaging/com.goshitsarch-eng.gosh-github-backup-manager.metainfo.xml \
    %{buildroot}%{_metainfodir}/com.goshitsarch-eng.gosh-github-backup-manager.metainfo.xml

# Install icons
install -Dm644 assets/32x32.png \
    %{buildroot}%{_datadir}/icons/hicolor/32x32/apps/com.goshitsarch-eng.gosh-github-backup-manager.png
install -Dm644 assets/128x128.png \
    %{buildroot}%{_datadir}/icons/hicolor/128x128/apps/com.goshitsarch-eng.gosh-github-backup-manager.png
install -Dm644 assets/128x128@2x.png \
    %{buildroot}%{_datadir}/icons/hicolor/256x256/apps/com.goshitsarch-eng.gosh-github-backup-manager.png

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/applications/com.goshitsarch-eng.gosh-github-backup-manager.desktop
%{_metainfodir}/com.goshitsarch-eng.gosh-github-backup-manager.metainfo.xml
%{_datadir}/icons/hicolor/*/apps/com.goshitsarch-eng.gosh-github-backup-manager.png

%post
/usr/bin/update-desktop-database &> /dev/null || :
/usr/bin/gtk-update-icon-cache %{_datadir}/icons/hicolor &> /dev/null || :

%postun
/usr/bin/update-desktop-database &> /dev/null || :
/usr/bin/gtk-update-icon-cache %{_datadir}/icons/hicolor &> /dev/null || :

%changelog
* Mon Mar 16 2026 Gosh-Its-Arch <gosh@example.com> - 2.0.0-1
- Migrated from Tauri/React to native Rust with iced GUI framework
- Removed all JavaScript/Node.js dependencies
- Pure Rust application with no web runtime overhead

* Sun Dec 29 2024 Gosh-Its-Arch <gosh@example.com> - 1.0.0-1
- Initial release
