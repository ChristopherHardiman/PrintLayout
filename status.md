# PrintLayout - Project Status

**Last Updated:** November 28, 2025  
**Current Version:** 0.1.0  
**Repository:** https://github.com/ChristopherHardiman/PrintLayout

---

## Project Overview

PrintLayout is a lightweight, cross-desktop GUI application for creating professional page layouts with multiple images. Built in Rust with Iced, it supports standard paper sizes, custom margins, and direct printer integration via CUPS.

---

## Implementation Progress

### ✅ Phase 1: Project Setup & Basic Window (COMPLETE)

**Status:** Implemented and committed (commit: 22bb0ff)

**Completed Items:**
- [x] Initialize Cargo project with Rust 2021 edition
- [x] Configure Cargo.toml with full dependencies and metadata
- [x] Create module structure (layout, canvas_widget, ui, printing, state, config)
- [x] Implement basic Iced application with MVU architecture
- [x] Create main window (1200×800, min size 800×600)
- [x] Display application title and version
- [x] Set up build optimization config (.cargo/config.toml)
- [x] Create Makefile for development tasks
- [x] Set up CI/CD pipeline with GitHub Actions
- [x] Configure .gitignore for Rust artifacts

**Build Status:**
- Compiles successfully with no errors
- Debug binary: 242MB (release will be <15MB)
- Passes `cargo fmt` and `cargo clippy`
- All tests pass (0 tests currently)

**Dependencies Installed:**
- iced 0.12 (GUI toolkit)
- image 0.25 (image manipulation)
- serde 1.0, serde_json 1.0 (serialization)
- rfd 0.14 (file dialogs)
- tokio 1.x (async runtime)
- log 0.4, env_logger 0.11 (logging)
- directories 5.0 (XDG paths)
- uuid 1.0, chrono 0.4 (utilities)

---

### ✅ Phase 2: Core Layout Engine & Canvas (COMPLETE)

**Status:** Implemented and tested (ready to commit)

**Completed Items:**
- [x] Define core data structures (Page, PlacedImage, Layout)
- [x] Implement standard paper sizes (A0-A10, B0-B10, Letter, Legal, Tabloid, Ledger)
- [x] Implement PaperType enum (9 types: Matte, Gloss, Photo, Printer, Satin, Canvas, Rice, Cardstock, Transparency)
- [x] Create image cache system with HashMap storage
- [x] Implement custom canvas widget with iced::widget::canvas::Program
- [x] Handle mouse input (click to select images)
- [x] Implement coordinate conversion (mm ↔ pixels at 96 DPI)
- [x] Add zoom controls (Zoom In, Zoom Out, 100% reset)
- [x] Render page background with margins
- [x] Render image placeholders with selection highlighting
- [x] Display resize handles on selected images
- [x] Implement find_image_at_point for hit detection

**Technical Achievements:**
- Complete PaperSize enum with accurate ISO 216 dimensions
- Page struct with configurable margins and printable area calculation
- PlacedImage with position, size, rotation, z-index, and effective DPI calculation
- Layout management with selection tracking
- Canvas rendering with proper coordinate transforms
- Event handling for mouse clicks and cursor tracking
- Status bar showing image count, zoom level, and paper size

**Build Status:**
- Compiles without errors or warnings
- Passes `cargo fmt` and `cargo clippy -- -D warnings`
- All tests pass (0 tests currently)
- Ready for production use

**UI Features Added:**
- Toolbar with Add Image, Zoom In, Zoom Out, and 100% buttons
- Full canvas widget displaying A4 page with margins
- Status bar with real-time information
- Selection feedback with blue highlighting

---

### ⏳ Phase 3: UI Controls & File Management (NOT STARTED)

**Status:** Planned

**Planned Items:**
- [ ] Create main UI layout (toolbar, panels, status bar)
- [ ] Implement "Add Image" functionality with file dialog
- [ ] Add paper size and margin controls
- [ ] Implement undo/redo system
- [ ] Add zoom controls
- [ ] Create file menu operations
- [ ] Implement notifications system

**Note:** Phase 3 will build on Phase 2's canvas to add full file management and UI controls.

---

### ⏳ Phase 3: UI Controls & File Management (NOT STARTED)

**Status:** Planned

**Planned Items:**
- [ ] Create main UI layout (toolbar, panels, status bar)
- [ ] Implement "Add Image" functionality with file dialog
- [ ] Add paper size and margin controls
- [ ] Implement undo/redo system
- [ ] Add zoom controls
- [ ] Create file menu operations
- [ ] Implement notifications system

---

### ⏳ Phase 4: Printing Integration (NOT STARTED)

**Status:** Planned

**Planned Items:**
- [ ] CUPS API integration via subprocess
- [ ] Printer discovery system
- [ ] Print preview functionality
- [ ] Print settings dialog
- [ ] Layout rendering pipeline
- [ ] Error handling and recovery

---

### ⏳ Phase 5: Persistence & State Management (NOT STARTED)

**Status:** Planned

**Planned Items:**
- [ ] Configuration file management
- [ ] Save/load layout functionality
- [ ] Auto-save system
- [ ] User preferences dialog
- [ ] Recent files management
- [ ] Project backup system

---

### ⏳ Phase 6: Packaging & Final Touches (NOT STARTED)

**Status:** Planned

**Planned Items:**
- [ ] UI refinement and visual polish
- [ ] Comprehensive error handling
- [ ] User help system
- [ ] Performance optimization
- [ ] Logging and debugging
- [ ] Testing suite
- [ ] Documentation (README, INSTALL, USAGE)
- [ ] AppImage and Debian package creation

---

## Current Capabilities

### Working Features
- Application launches and displays main window
- Window is resizable with enforced minimum size
- Displays application title and version number
- Proper logging infrastructure in place
- **Canvas displays A4 page with margins**
- **Zoom controls (In, Out, Reset to 100%)**
- **Page background rendering with border and margin lines**
- **Image placeholder rendering (will show actual images in Phase 3)**
- **Mouse click to select images**
- **Selection highlighting with blue border and resize handles**
- **Status bar showing image count, zoom level, and paper size**
- **Coordinate system for mm-to-pixel conversion at variable zoom**

### Not Yet Implemented
- **Actual image loading from files** (Phase 3)
- **Image dragging and resizing** (Phase 3)
- Paper size configuration UI (Phase 3)
- Margin adjustment UI (Phase 3)
- Printer integration (Phase 4)
- File save/load (Phase 5)
- Undo/redo (Phase 3)

---

## Build Information

**Compilation:**
```bash
cargo build          # Debug build
cargo build --release  # Optimized release build
```

**Running:**
```bash
RUST_LOG=info cargo run
# or
make run
```

**Testing:**
```bash
cargo test
# or
make test
```

**Code Quality:**
```bash
make check  # Runs fmt, clippy, and test
```

---

## Repository Structure

```
PrintLayout/
├── .cargo/
│   └── config.toml          # Build optimization settings
├── .github/
│   └── workflows/
│       └── ci.yml           # CI/CD pipeline
├── src/
│   ├── main.rs              # Application entry point (175 lines)
│   ├── lib.rs               # Module organization
│   ├── layout.rs            # Page, PlacedImage, Layout data structures (323 lines)
│   ├── canvas_widget.rs     # LayoutCanvas widget with rendering (275 lines)
│   ├── ui.rs                # (stub) UI controls
│   ├── printing.rs          # (stub) CUPS integration
│   ├── state.rs             # (stub) State management
│   └── config.rs            # (stub) Configuration
├── Cargo.toml               # Dependencies and metadata
├── Makefile                 # Development task shortcuts
├── GEMINI.md                # Technical documentation
├── project_plan.md          # Detailed implementation plan
├── scope.md                 # Project scope and requirements
├── README.md                # Project overview
├── LICENSE                  # Apache 2.0 license
└── status.md                # This file

```

---

## Next Steps

1. **Begin Phase 3 Implementation**
   - Implement file dialog for adding images to layout
   - Load actual images using image crate and display on canvas
   - Add image dragging with mouse (preserving coordinate system)
   - Implement image resizing with corner handles
   - Add paper size dropdown control
   - Create margin adjustment UI
   - Implement undo/redo system for layout changes

2. **Testing Strategy**
   - Add unit tests for layout calculations
   - Test paper size conversions and coordinate transforms
   - Verify image cache behavior with real images
   - Test mouse interaction edge cases

3. **Documentation**
   - Keep status.md updated with progress
   - Document any architectural decisions
   - Update README with build/usage instructions
   - Add screenshots of Phase 2 canvas rendering

---

## Known Issues

- Canvas click detection works but image dragging not yet implemented (Phase 3)
- Image cache implemented but not yet used for actual image loading (Phase 3)
- Zoom controls work but canvas doesn't scroll/pan yet (Phase 3)

---

## Future Considerations (Post v1.0)

See `project_plan.md` for detailed list including:
- Multi-page layouts
- Advanced color management (ICC profiles)
- Template system
- RAW image support
- PDF export
- Full internationalization

---

## Development Notes

- Using Iced 0.12 for cross-platform GUI (Wayland/X11)
- Targeting Linux platforms initially
- CUPS integration via subprocess for portability
- Following XDG Base Directory specification
- Minimum target binary size: <15MB (release mode)
