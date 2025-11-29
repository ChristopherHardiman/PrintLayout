Name:           print-layout
Version:        0.1.0
Release:        1%{?dist}
Summary:        Lightweight cross-desktop GUI for creating print layouts

License:        Apache-2.0
URL:            https://github.com/ChristopherHardiman/PrintLayout
Source0:        %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  rust >= 1.75
BuildRequires:  cargo
BuildRequires:  cups-devel
BuildRequires:  libxkbcommon-devel
BuildRequires:  wayland-devel
BuildRequires:  libX11-devel

Requires:       cups
Requires:       xdg-desktop-portal
Requires:       libxkbcommon

%description
Print Layout is a lightweight, cross-desktop GUI application for creating 
professional page layouts with multiple images. Built in Rust with the Iced 
GUI toolkit, it supports standard paper sizes, custom margins, and direct 
printer integration via CUPS.

Features include:
- Add and arrange multiple images on a page
- Drag-to-resize with 8 resize handles
- Image manipulation (rotate, flip, opacity)
- Support for A-series, B-series, Letter, Legal, and photo paper sizes
- Portrait and landscape orientation
- CUPS printer discovery and high-resolution printing
- Auto-save and project backup system

%prep
%autosetup -n PrintLayout-%{version}

%build
cargo build --release

%install
# Create directories
install -d %{buildroot}%{_bindir}
install -d %{buildroot}%{_datadir}/applications
install -d %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
install -d %{buildroot}%{_datadir}/icons/hicolor/256x256/apps
install -d %{buildroot}%{_datadir}/licenses/%{name}
install -d %{buildroot}%{_mandir}/man1

# Install binary
install -m 0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install desktop file
install -m 0644 assets/print-layout.desktop %{buildroot}%{_datadir}/applications/%{name}.desktop

# Install icons
install -m 0644 assets/icons/print-layout.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/%{name}.svg

# Install license
install -m 0644 LICENSE %{buildroot}%{_datadir}/licenses/%{name}/LICENSE

%post
# Update icon cache
/usr/bin/touch --no-create %{_datadir}/icons/hicolor &>/dev/null || :
# Update desktop database
/usr/bin/update-desktop-database &>/dev/null || :

%postun
if [ $1 -eq 0 ]; then
    /usr/bin/touch --no-create %{_datadir}/icons/hicolor &>/dev/null || :
fi
# Update desktop database
/usr/bin/update-desktop-database &>/dev/null || :

%files
%license LICENSE
%{_bindir}/%{name}
%{_datadir}/applications/%{name}.desktop
%{_datadir}/icons/hicolor/scalable/apps/%{name}.svg
%{_datadir}/licenses/%{name}/LICENSE

%changelog
* Thu Nov 28 2024 Christopher Hardiman <christopher.m.hardiman@gmail.com> - 0.1.0-1
- Initial release
- Image management with drag-and-drop positioning
- Drag-to-resize with 8 handles (corners and edges)
- Image manipulation: rotate, flip, resize, opacity
- Standard and photo paper sizes support
- Portrait/Landscape orientation
- CUPS printer integration
- Project save/load with auto-save
- Configuration persistence
