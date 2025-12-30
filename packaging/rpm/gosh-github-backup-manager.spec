Name:           gosh-github-backup-manager
Version:        1.0.0
Release:        1%{?dist}
Summary:        A cross-platform app to backup your GitHub repositories

License:        AGPL-3.0
URL:            https://github.com/Gosh-Its-Arch/Github-Backup-Manager
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  nodejs >= 18
BuildRequires:  npm
BuildRequires:  rust >= 1.70
BuildRequires:  cargo
BuildRequires:  webkit2gtk4.1-devel
BuildRequires:  gtk3-devel
BuildRequires:  openssl-devel
BuildRequires:  libappindicator-gtk3-devel
BuildRequires:  librsvg2-devel
BuildRequires:  curl
BuildRequires:  wget
BuildRequires:  file

Requires:       git
Requires:       webkit2gtk4.1
Requires:       gtk3
Requires:       openssl-libs
Requires:       libappindicator-gtk3

%description
Gosh Github Backup Manager is a modern desktop application that helps you
backup your GitHub repositories to your local machine. It provides an
intuitive interface for managing your repository backups with support for
multiple backup modes including clone and mirror.

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
npm ci
npm run build

%install
rm -rf %{buildroot}

# Install binary
install -Dm755 src-tauri/target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install desktop file
install -Dm644 packaging/com.goshitsarch-eng.gosh-github-backup-manager.desktop \
    %{buildroot}%{_datadir}/applications/com.goshitsarch-eng.gosh-github-backup-manager.desktop

# Install metainfo
install -Dm644 packaging/com.goshitsarch-eng.gosh-github-backup-manager.metainfo.xml \
    %{buildroot}%{_metainfodir}/com.goshitsarch-eng.gosh-github-backup-manager.metainfo.xml

# Install icons
install -Dm644 src-tauri/icons/32x32.png \
    %{buildroot}%{_datadir}/icons/hicolor/32x32/apps/com.goshitsarch-eng.gosh-github-backup-manager.png
install -Dm644 src-tauri/icons/128x128.png \
    %{buildroot}%{_datadir}/icons/hicolor/128x128/apps/com.goshitsarch-eng.gosh-github-backup-manager.png
install -Dm644 src-tauri/icons/128x128@2x.png \
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
* Sun Dec 29 2024 Gosh-Its-Arch <gosh@example.com> - 1.0.0-1
- Initial release
- GitHub authentication via Personal Access Token
- Repository browsing and filtering
- Backup with clone and mirror modes
- ZIP archive creation support
- Light and dark theme support
