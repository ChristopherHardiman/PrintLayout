# GEMINI.md

## Project Overview

This project is a graphical user interface (GUI) application for creating and printing page layouts on Linux. It allows users to intuitively arrange one or more images on a virtual page, configure page size, paper type, and margins, and then send the final layout to a printer.

The application is designed to be lightweight, performant, and independent of any specific desktop environment.

### Core Technologies

*   **Programming Language:** Rust
*   **GUI Toolkit:** Iced (`iced` crate)
*   **Image Manipulation:** `image` crate
*   **Printing System:** CUPS (via `cups-rs` or `subprocess`)
*   **Serialization:** `serde` and `serde_json` for saving/loading layouts and preferences.

### Architecture

The application follows the Model-View-Update (MVU) architecture inherent to the Iced toolkit. The state is managed in a central `PrintLayout` struct, and user interactions are handled through a `Message` enum. The project is organized into a modular structure with clear separation of concerns for layout logic, canvas rendering, UI controls, and printing.

## Building and Running

The project uses the standard Rust toolchain (Cargo).

### Building

To build an optimized release binary:
```bash
cargo build --release
```

### Running

After a successful build, the application can be run from the target directory:
```bash
./target/release/print_layout
```

### Testing

To run the test suite:
```bash
cargo test
```

## Development Conventions

*   **Code Style:** The project adheres to standard Rust formatting, enforced by `cargo fmt`.
*   **Linting:** Code quality is maintained using `cargo clippy`.
*   **Dependencies:** All dependencies are managed in `Cargo.toml`.
*   **Modularity:** The codebase is structured into modules as outlined in `project_plan.md` (e.g., `src/layout.rs`, `src/canvas.rs`, `src/ui.rs`).
*   **Automation:** Common development tasks (build, run, test, clean) can be automated using a `Makefile` or shell scripts as suggested in the project plan.
*   **Configuration:** User-specific configuration is stored in `~/.config/print_layout/`, following the XDG Base Directory Specification.
*   **Project Files:**
    *   `scope.md`: Contains the high-level project goals and technical specifications.
    *   `project_plan.md`: Provides a detailed, multi-phase implementation plan.
