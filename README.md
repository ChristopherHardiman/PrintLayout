# Print Layout

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A lightweight, cross-desktop GUI application for creating professional page layouts with multiple images. Built in Rust with the [Iced](https://iced.rs/) GUI toolkit, Print Layout supports standard paper sizes, custom margins, and direct printer integration via CUPS.

## ✨ Features

### 🖼️ Image Management
- Add multiple images (PNG, JPEG, GIF, BMP, WebP)
- Drag-and-drop positioning on canvas
- **Drag-to-resize** with 8 handles (corners and edges)
- Visual selection highlighting with resize handles
- Thumbnail panel for quick image selection

### 🔄 Image Manipulation
- **Rotate** 90° clockwise/counter-clockwise
- **Flip** horizontal and vertical
- **Resize** with width/height inputs and aspect ratio lock
- **Opacity** control (0-100%)
- Live preview of all transformations

### 📄 Page Configuration
- **Standard paper sizes**: A-series (A3-A5), B-series, Letter, Legal, Tabloid, Ledger
- **Photo paper sizes**: 3.5×5" through 13×19" (A3+), Panorama
- **Paper types**: Plain, Super High Gloss, Glossy, Semi-Gloss, Matte, Fine Art
- Custom margin controls (Top, Bottom, Left, Right)
- Portrait/Landscape orientation toggle
- Borderless printing option

### 🖨️ Printing
- CUPS printer discovery and selection
- High-resolution rendering (300 DPI default)
- Print quality presets (Draft, Standard, High, Highest)
- Color mode selection (ICC Profile, Driver Matching, B&W)
- Full transform support in print output

### 💾 Project Management
- Save/Load layouts (`.pxl` format)
- Auto-save every 30 seconds
- Automatic backup system (keeps 5 most recent)
- Recent files menu with quick access
- Auto-save recovery on startup
- Dirty state indicator (*) in window title

## 🚀 Installation

### Fedora / RHEL / CentOS Stream

```bash
# Enable COPR repository (coming soon)
sudo dnf copr enable christopherhardiman/print-layout
sudo dnf install print-layout
```

### Build from Source

See [INSTALL.md](INSTALL.md) for detailed build instructions.

```bash
# Install dependencies (Fedora)
sudo dnf install rust cargo cups-devel gtk3-devel libxkbcommon-devel

# Clone and build
git clone https://github.com/ChristopherHardiman/PrintLayout.git
cd PrintLayout
cargo build --release

# Run
./target/release/print_layout
```

### AppImage (Universal Linux)

Download the latest AppImage from the [Releases](https://github.com/ChristopherHardiman/PrintLayout/releases) page:

```bash
chmod +x print-layout-*.AppImage
./print-layout-*.AppImage
```

## 📖 Quick Start

1. **Launch** the application
2. **Add images** using the "+ Add Image" button or drag files onto the canvas
3. **Position images** by dragging them on the canvas
4. **Resize images** using the corner and edge handles
5. **Adjust settings** in the right panel (paper size, margins, print quality)
6. **Save your layout** with Ctrl+S or File → Save
7. **Print** using the Print button when ready

## ⌨️ Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| New Layout | Ctrl+N |
| Open Layout | Ctrl+O |
| Save Layout | Ctrl+S |
| Save As | Ctrl+Shift+S |
| Add Image | Click "+ Add Image" |
| Delete Selected | Delete |
| Zoom In | Ctrl++ |
| Zoom Out | Ctrl+- |
| Zoom Reset | Ctrl+0 |
| Fit to Window | Ctrl+F |

## 🔧 System Requirements

- **OS**: Linux (X11 or Wayland)
- **CPU**: x86_64 or ARM64
- **RAM**: 512 MB minimum, 2 GB recommended
- **Disk**: 50 MB for application, additional space for layouts
- **Dependencies**: CUPS (for printing), GTK3 (for file dialogs)

## 📁 File Locations

| File Type | Location |
|-----------|----------|
| Configuration | `~/.config/print_layout/config.json` |
| Auto-save | `~/.cache/print_layout/auto_save.pxl` |
| Backups | `~/.config/print_layout/backups/` |
| Logs | `~/.cache/print_layout/app.log` |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Iced](https://iced.rs/) - Cross-platform GUI library for Rust
- [image-rs](https://github.com/image-rs/image) - Image processing library
- [rfd](https://github.com/PolyMeilex/rfd) - Rusty File Dialog

## 📬 Support

- **Issues**: [GitHub Issues](https://github.com/ChristopherHardiman/PrintLayout/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ChristopherHardiman/PrintLayout/discussions)
