# GEMINI.md

## Project Overview

This project is a graphical user interface (GUI) application for creating and printing page layouts on Linux. It allows users to intuitively arrange one or more images on a virtual page, configure page size, paper type, and margins, and then send the final layout to a printer.

The application is designed to be lightweight, performant, and independent of any specific desktop environment.

### Core Technologies

*   **Programming Language:** Rust
*   **GUI Toolkit:** Iced (`iced` crate) - cross-desktop, runs on Wayland and X11
*   **Image Manipulation:** `image` crate (PNG, JPEG, GIF, BMP, WebP, TIFF support)
*   **Printing System:** CUPS (via `lp`/`lpstat` subprocess commands for portability)
*   **Serialization:** `serde` and `serde_json` for saving/loading layouts and preferences
*   **Async Runtime:** `tokio` for background operations
*   **File Dialogs:** `rfd` crate for native file picker dialogs
*   **Logging:** `log` and `env_logger` crates
*   **Configuration:** `directories` crate for XDG-compliant paths

### Architecture

The application follows the Model-View-Update (MVU) architecture inherent to the Iced toolkit. The state is managed in a central `PrintLayout` struct, and user interactions are handled through a `Message` enum. The project is organized into a modular structure with clear separation of concerns for layout logic, canvas rendering, UI controls, and printing.

**Module Structure:**
*   `src/main.rs` - Application entry point and main state
*   `src/lib.rs` - Module organization
*   `src/layout.rs` - Page and image data structures
*   `src/canvas.rs` - Canvas widget and image cache
*   `src/ui.rs` - UI controls and layouts
*   `src/printing.rs` - CUPS integration
*   `src/state.rs` - Application state management
*   `src/config.rs` - Configuration and user preferences

## Building and Running

The project uses the standard Rust toolchain (Cargo).

### Prerequisites

*   Rust 1.70+ (install via [rustup](https://rustup.rs/))
*   CUPS installed and running (for printing)
*   Linux with X11 or Wayland display server

### Building

To build an optimized release binary:
```bash
cargo build --release
```

The release build includes optimizations:
*   LTO (Link-Time Optimization)
*   Single codegen unit
*   Binary stripping
*   Target binary size: <15MB

### Running

After a successful build, the application can be run from the target directory:
```bash
./target/release/print_layout
```

**Command-line options:**
```bash
./target/release/print_layout --debug    # Enable debug logging
./target/release/print_layout --trace    # Enable trace-level logging
```

### Testing

To run the test suite:
```bash
cargo test
```

To run with coverage (requires `cargo-tarpaulin`):
```bash
cargo tarpaulin --out Html
```

### Linting and Formatting

```bash
cargo fmt --check    # Check formatting
cargo fmt            # Apply formatting
cargo clippy         # Run linter
```

## Development Conventions

*   **Code Style:** The project adheres to standard Rust formatting, enforced by `cargo fmt`.
*   **Linting:** Code quality is maintained using `cargo clippy`.
*   **Dependencies:** All dependencies are managed in `Cargo.toml`. Pin major versions to avoid breaking changes.
*   **Modularity:** The codebase is structured into modules as outlined in `project_plan.md` (e.g., `src/layout.rs`, `src/canvas.rs`, `src/ui.rs`).
*   **Automation:** Common development tasks (build, run, test, clean) can be automated using a `Makefile` or shell scripts as suggested in the project plan.
*   **CI/CD:** GitHub Actions workflow for automated testing, linting, and release builds.
*   **Configuration:** User-specific configuration is stored in `~/.config/print_layout/`, following the XDG Base Directory Specification.
*   **Cache/Temp Files:** Stored in `~/.cache/print_layout/` for auto-saves, logs, and temporary print files.
*   **Project Files:**
    *   `scope.md`: Contains the high-level project goals and technical specifications.
    *   `project_plan.md`: Provides a detailed, multi-phase implementation plan.

## Key Features

*   **Multi-image layouts** with drag-and-drop, move, resize, and rotate
*   **Smart snapping** to grid, other images, and page boundaries
*   **Multi-selection** for batch operations
*   **Standard paper sizes** (A-series, B-series, Letter, Legal, etc.) with regional defaults
*   **Custom paper sizes** with mm/inch units
*   **Margin controls** with presets and linked margins option
*   **DPI quality indicators** showing print resolution warnings
*   **Print preview** before sending to printer
*   **Undo/redo** with history management
*   **Auto-save** and crash recovery
*   **Project save/load** with relative path support

## Security Considerations

*   All file paths are sanitized before use
*   Printer names are validated to prevent command injection
*   Temporary files are created with secure permissions (600)
*   Proper cleanup on all exit paths including crashes
*   Config files stored with appropriate permissions

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Add Image | Ctrl+I |
| Open Layout | Ctrl+O |
| Save Layout | Ctrl+S |
| Save As | Ctrl+Shift+S |
| Undo | Ctrl+Z |
| Redo | Ctrl+Shift+Z |
| Select All | Ctrl+A |
| Copy | Ctrl+C |
| Paste | Ctrl+V |
| Duplicate | Ctrl+D |
| Delete | Delete |
| Zoom In | Ctrl+= |
| Zoom Out | Ctrl+- |
| Fit to Window | Ctrl+0 |
| Move (fine) | Arrow Keys |
| Move (coarse) | Shift+Arrow Keys |
| Rotate | R / Shift+R |
| Lock/Unlock | L |
| Toggle Visibility | H |
| Deselect All | Escape |
