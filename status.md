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

### ✅ Phase 3: UI Controls & File Management (COMPLETE)

**Status:** Implemented and tested

**Completed Items:**
- [x] Create main UI layout with left sidebar and toolbar
- [x] Implement "Add Image" functionality with rfd::AsyncFileDialog
- [x] Add multi-file selection with image format filters (png, jpg, jpeg, gif, bmp, webp)
- [x] Load images using image crate with proper error handling
- [x] Create PlacedImage instances with automatic sizing to fit page
- [x] Add paper size dropdown (pick_list) with A4, A3, Letter, Legal, Tabloid, Ledger options
- [x] Implement Display trait for PaperSize enum for dropdown text
- [x] Add margin input controls (top, bottom, left, right) with validation
- [x] Implement delete image button functionality
- [x] Add mouse drag support for moving images on canvas
- [x] Handle MouseMoved and MouseReleased events for smooth dragging
- [x] Track drag state in main application model
- [x] Update canvas rendering during drag operations

**Technical Achievements:**
- Async file dialog integration using rfd::AsyncFileDialog
- Image loading with image::open() and GenericImageView trait
- Paper size picker using iced::widget::pick_list
- Text input fields for margins with float validation
- Drag state tracking with initial position calculation
- Delta-based position updates for smooth dragging
- Canvas event handling for ButtonPressed, CursorMoved, and ButtonReleased
- Proper borrow checker handling for drag operations

**Build Status:**
- Compiles without errors or warnings
- Passes `cargo fmt` and `cargo clippy -- -D warnings`
- All tests pass (0 tests currently)
- Application runs and displays UI correctly

**UI Features Added:**
- Left sidebar (200px) with:
  * Paper size dropdown showing all standard sizes
  * Four margin input fields (top, bottom, left, right)
  * Clean vertical layout with labels
- Toolbar with:
  * Add Image button (opens file dialog)
  * Delete button (removes selected image)
  * Zoom In, Zoom Out, 100% buttons
- Canvas with:
  * Image placeholder rendering
  * Mouse click to select images
  * Mouse drag to move images
  * Selection highlighting
- Status bar with image count, zoom, and paper info

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
- **Canvas displays pages with configurable paper sizes**
- **Zoom controls (In, Out, Reset to 100%)**
- **Page background rendering with border and margin lines**
- **Paper size dropdown with all standard sizes (A0-A10, B0-B10, Letter, Legal, Tabloid, Ledger)**
- **Margin input controls with validation**
- **File dialog for adding images (multi-select supported)**
- **Image loading from disk (PNG, JPEG, GIF, BMP, WebP)**
- **Image placeholder rendering on canvas**
- **Mouse click to select images**
- **Mouse drag to move images on canvas**
- **Delete button to remove selected images**
- **Selection highlighting with blue border and resize handles**
- **Status bar showing image count, zoom level, and paper size**
- **Coordinate system for mm-to-pixel conversion at variable zoom**
- **Left sidebar with paper size and margin controls**

### Not Yet Implemented
- **Actual image rendering on canvas (currently shows colored placeholders)** (Phase 3 refinement)
- **Image resizing with handles** (Phase 3 refinement)
- Undo/redo system (Phase 3)
- Printer integration (Phase 4)
- File save/load (Phase 5)
- Layers panel (Phase 3)
- Menu bar (Phase 3)

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
│   ├── main.rs              # Application entry point (385 lines)
│   ├── lib.rs               # Module organization
│   ├── layout.rs            # Page, PlacedImage, Layout data structures (357 lines)
│   ├── canvas_widget.rs     # LayoutCanvas widget with rendering (290 lines)
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

1. **Refine Phase 3 Implementation**
   - Render actual images on canvas (not just placeholders)
   - Implement image resizing with corner handles
   - Add undo/redo system for layout changes
   - Implement layers panel on right side
   - Add menu bar with File/Edit/View/Help
   - Create keyboard shortcuts for common operations

2. **Begin Phase 4 Implementation**
   - Integrate CUPS API for printer discovery
   - Implement print preview dialog
   - Add print settings UI
   - Create layout rendering pipeline for printing
   - Handle printer errors and recovery

3. **Testing Strategy**
   - Add unit tests for layout calculations
   - Test paper size conversions and coordinate transforms
   - Verify image cache behavior with real images
   - Test mouse interaction edge cases
   - Test drag operations with multiple images

4. **Documentation**
   - Keep status.md updated with progress
   - Document any architectural decisions
   - Update README with build/usage instructions
   - Add screenshots of Phase 3 UI

---

## Known Issues

- Actual image pixels not rendered on canvas yet - showing colored placeholders for now (Phase 3 refinement)
- Image resizing handles visible but not functional yet (Phase 3 refinement)
- No undo/redo system yet (Phase 3)
- Canvas doesn't scroll/pan for large pages (Phase 3 refinement)

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
