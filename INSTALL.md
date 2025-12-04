# Installation Guide

This document provides detailed instructions for installing Print Layout on various Linux distributions.

## System Requirements

### Minimum Requirements
- **OS**: Linux with X11 or Wayland display server
- **CPU**: x86_64 or ARM64 (aarch64)
- **RAM**: 512 MB
- **Disk Space**: 50 MB for application
- **Display**: 1024x768 minimum resolution

### Recommended Requirements
- **RAM**: 2 GB or more
- **Display**: 1920x1080 or higher
- **Storage**: SSD for faster layout loading

### Runtime Dependencies
- **CUPS** - Required for printing functionality
- **xdg-desktop-portal** - Required for native file dialogs
- **libxkbcommon** - Required for keyboard input

## Installation Methods

### Method 1: Fedora / RHEL / CentOS Stream (RPM)

#### From COPR Repository (Recommended)
```bash
# Enable the COPR repository
sudo dnf copr enable chrismhardiman/print-layout

# Install Print Layout
sudo dnf install print-layout
```

#### Manual RPM Installation
```bash
# Download the RPM package from releases
wget https://github.com/ChristopherHardiman/PrintLayout/releases/download/v0.2.1/print-layout-0.2.1-1.fc43.x86_64.rpm

# Install with dnf
sudo dnf install ./print-layout-0.2.1-1.fc43.x86_64.rpm
```

### Method 2: AppImage (Universal Linux)

AppImage works on any Linux distribution without installation:

```bash
# Download the AppImage
wget https://github.com/ChristopherHardiman/PrintLayout/releases/download/v0.2.1/print-layout-0.2.1-x86_64.AppImage

# Make it executable
chmod +x print-layout-0.2.1-x86_64.AppImage

# Run the application
./print-layout-0.2.1-x86_64.AppImage
```

#### Optional: Desktop Integration for AppImage
```bash
# Install AppImageLauncher for automatic desktop integration
# Or manually create a desktop entry:
mkdir -p ~/.local/share/applications
cat > ~/.local/share/applications/print-layout.desktop << EOF
[Desktop Entry]
Type=Application
Name=Print Layout
Exec=/path/to/print-layout-0.2.1-x86_64.AppImage
Icon=print-layout
Categories=Graphics;Photography;Publishing;
EOF
```

### Method 3: Build from Source

#### Prerequisites

##### Fedora / RHEL / CentOS Stream
```bash
sudo dnf install rust cargo cups-devel libxkbcommon-devel \
    wayland-devel libX11-devel git
```

##### Ubuntu / Debian
```bash
sudo apt update
sudo apt install rustc cargo libcups2-dev libxkbcommon-dev \
    libwayland-dev libx11-dev git build-essential
```

##### Arch Linux
```bash
sudo pacman -S rust cups xdg-desktop-portal libxkbcommon wayland libx11 git base-devel
```

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/ChristopherHardiman/PrintLayout.git
cd PrintLayout

# Build release version
cargo build --release

# The binary will be at:
# ./target/release/print_layout
```

#### Install Manually (After Building)

```bash
# Install binary
sudo install -m 755 target/release/print_layout /usr/local/bin/

# Install desktop file
sudo install -m 644 assets/print-layout.desktop /usr/share/applications/

# Install icon
sudo install -m 644 assets/icons/print-layout.svg /usr/share/icons/hicolor/scalable/apps/

# Update icon cache
sudo touch /usr/share/icons/hicolor
```

## Verifying Installation

After installation, verify that Print Layout works correctly:

```bash
# Check version
print-layout --version

# Run the application
print-layout
```

## Post-Installation Setup

### Printer Configuration

Print Layout uses CUPS for printing. Ensure CUPS is running:

```bash
# Check CUPS status
systemctl status cups

# Start CUPS if not running
sudo systemctl start cups
sudo systemctl enable cups
```

### Desktop Integration

After installation, Print Layout should appear in your application menu under:
- **Graphics** category
- **Photography** category
- **Publishing** category

You can also search for "Print Layout" in your desktop environment's application launcher.

## Troubleshooting

### Application Won't Start

1. **Check dependencies**:
   ```bash
   ldd /usr/bin/print-layout | grep "not found"
   ```

2. **Run from terminal** to see error messages:
   ```bash
   print-layout 2>&1
   ```

### No Printers Detected

1. **Verify CUPS is running**:
   ```bash
   systemctl status cups
   ```

2. **Check printer list**:
   ```bash
   lpstat -p
   ```

3. **Ensure user is in lp group**:
   ```bash
   sudo usermod -aG lp $USER
   # Log out and back in for changes to take effect
   ```

### File Dialog Not Opening

This usually indicates a missing xdg-desktop-portal dependency:

```bash
# Fedora
sudo dnf install xdg-desktop-portal xdg-desktop-portal-gtk

# Ubuntu/Debian
sudo apt install xdg-desktop-portal xdg-desktop-portal-gtk
```

### Wayland Issues

If you experience issues on Wayland, try running with X11:

```bash
GDK_BACKEND=x11 print-layout
```

## Uninstallation

### Fedora / RHEL (RPM)
```bash
sudo dnf remove print-layout
```

### Manual Installation
```bash
sudo rm /usr/local/bin/print-layout
sudo rm /usr/share/applications/print-layout.desktop
sudo rm /usr/share/icons/hicolor/scalable/apps/print-layout.svg
rm -rf ~/.config/print_layout
rm -rf ~/.cache/print_layout
```

## Getting Help

- **Documentation**: See [README.md](README.md) for usage information
- **Issues**: Report bugs at [GitHub Issues](https://github.com/ChristopherHardiman/PrintLayout/issues)
- **Discussions**: Ask questions at [GitHub Discussions](https://github.com/ChristopherHardiman/PrintLayout/discussions)
