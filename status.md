# PrintLayout - Project Status

**Last Updated:** December 2025  
**Current Version:** 0.2.1  
**Repository:** https://github.com/ChristopherHardiman/PrintLayout

---

## Project Overview

PrintLayout is a lightweight, cross-desktop GUI application for creating professional page layouts with multiple images. Built in Rust with Iced 0.13, it supports standard paper sizes, custom margins, and direct printer integration via CUPS.

---

## Implementation Progress

### ✅ Phase 1: Project Setup & Basic Window (COMPLETE)

- Cargo project with Rust 2021 edition
- Full dependency configuration
- Module structure (layout, canvas_widget, printing, config)
- Basic Iced application with MVU architecture
- Main window (1200×800, min size 800×600)
- Build optimization config, Makefile, CI/CD pipeline

---

### ✅ Phase 2: Core Layout Engine & Canvas (COMPLETE)

- Core data structures (Page, PlacedImage, Layout)
- Standard paper sizes (A0-A10, B0-B10, Letter, Legal, Tabloid, Ledger)
- Photo paper sizes (3.5×5" through 13×19")
- Paper types (Plain, Glossy, Semi-Gloss, Matte, Fine Art, etc.)
- Image cache system with HashMap storage
- Custom canvas widget with Iced 0.13 image rendering
- Zoom controls, coordinate conversion (mm ↔ pixels)
- Selection highlighting and resize handles

---

### ✅ Phase 3: UI Controls & File Management (COMPLETE)

- Add Image functionality with async file dialog
- Multi-file selection with image format filters
- Paper size dropdown and margin controls
- Delete image functionality
- Mouse drag support for moving images on canvas
- Image loading and caching

---

### ✅ Phase 4: Printing Integration (COMPLETE)

- CUPS API integration via subprocess (lpstat, lp commands)
- Printer discovery and selection UI
- Print quality presets (Draft, Standard, High, Highest)
- Color mode selection (ICC Profile, Driver Matching, B&W)
- High-resolution rendering at 300 DPI
- Full transform support in print output
- Async print job execution with status callbacks

---

### ✅ Phase 5: Persistence & State Management (COMPLETE)

- Configuration persistence with XDG compliance
- Save/Load layouts (.pxl JSON format)
- Automatic backup system (keeps 5 most recent)
- Auto-save every 30 seconds
- Auto-save recovery dialog on startup
- Recent files tracking (up to 10 files)
- Dirty state tracking with (*) indicator in title

---

### ✅ Phase 6: Canon PPL-Style UI & Image Manipulation (COMPLETE)

- Major UI redesign matching Canon Professional Print & Layout
- Tabbed settings panel (Print Settings, Layout, Color, Image Tools)
- Thumbnail panel with horizontal scrolling
- Image manipulation tools:
  - Rotate 90° clockwise/counter-clockwise
  - Flip horizontal/vertical
  - Resize with aspect ratio lock
  - Opacity control (0-100%)
- Drag-to-resize with 8 handles (corners and edges)
- Transform-based image caching for performance

---

### ✅ Phase 7: Packaging & Final Touches (COMPLETE)

- RPM packaging for Fedora/RHEL/CentOS (`packaging/rpm/print-layout.spec`)
- GitHub Actions workflow for CI (`ci.yml`)
- GitHub Actions workflow for automated RPM releases (`release-rpm.yml`)
- Comprehensive documentation (README, INSTALL, USAGE, CHANGELOG)
- Local RPM build tested and working
- v0.2.0 release tagged

---

## Current Capabilities

### ✅ Working Features
- Full Canon PPL-inspired UI with tabbed settings panel
- Image management (add, delete, position, resize, select)
- All image transforms (rotate, flip, opacity) with canvas preview
- Complete paper size and type options
- Configurable margins with borderless option
- CUPS printer discovery and printing
- Save/Load layouts with auto-save and recovery
- Backup system and recent files tracking
- High-resolution printing at 300 DPI
- Transform-based caching for performance
- xdg-desktop-portal file dialogs (no GTK3 dependency)

### 📋 Planned for Future Releases
- Multi-page layout support
- PDF export
- Undo/redo system
- Template system
- Advanced ICC color profile support
- RAW image format support
- Batch printing with copy count

---

## Build Information

```bash
# Build
cargo build --release

# Run
./target/release/print_layout

# Build RPM locally
rpmbuild -ba packaging/rpm/print-layout.spec
```

---

## Repository Structure

```
PrintLayout/
├── .cargo/config.toml       # Build optimization
├── .github/workflows/
│   ├── ci.yml               # CI pipeline
│   └── release-rpm.yml      # Automated RPM releases
├── src/
│   ├── main.rs              # Application entry (1803 lines)
│   ├── lib.rs               # Module organization
│   ├── layout.rs            # Data structures (490 lines)
│   ├── canvas_widget.rs     # Canvas widget (520 lines)
│   ├── printing.rs          # CUPS integration (419 lines)
│   └── config.rs            # Persistence (295 lines)
├── packaging/rpm/
│   └── print-layout.spec    # RPM spec file
├── Cargo.toml               # Dependencies
├── Makefile                 # Development tasks
├── README.md                # Project overview
├── INSTALL.md               # Installation guide
├── USAGE.md                 # User documentation
├── CHANGELOG.md             # Version history
└── LICENSE                  # Apache 2.0
```

---

## File Locations

| File Type | Location |
|-----------|----------|
| Configuration | `~/.config/print_layout/config.json` |
| Auto-save | `~/.cache/print_layout/auto_save.pxl` |
| Backups | `~/.config/print_layout/backups/` |

---

## Known Limitations

- Single page layouts only (multi-page planned)
- No PDF export yet
- No undo/redo system
- RAW/HEIC image formats not supported
- Canvas doesn't scroll/pan for very large pages

---

## Development Notes

- Iced 0.13.1 with canvas, image, tokio features
- rfd 0.15 with xdg-portal (no GTK3 dependency)
- CUPS integration via subprocess for portability
- XDG Base Directory specification compliance
- Target binary size: ~7MB (release mode)
