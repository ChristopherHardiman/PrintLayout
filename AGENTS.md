# Repository Guidelines

## Project Structure & Module Organization

This is a Rust GUI application built with the Iced framework for creating professional print layouts. The main source code is organized in [src/](src/) with the following modules:

- [src/main.rs](src/main.rs) — Application entry point, UI state management, and message handling
- [src/layout.rs](src/layout.rs) — Core data structures (PaperSize, PlacedImage, Layout) and layout logic
- [src/canvas_widget.rs](src/canvas_widget.rs) — Custom canvas widget for image manipulation and drag-resize operations
- [src/config.rs](src/config.rs) — Configuration management, project persistence (.pxl format), and user preferences
- [src/printing.rs](src/printing.rs) — CUPS printer integration, capabilities querying, and print job execution

Assets are in [assets/](assets/), and packaging configurations in [packaging/rpm/](packaging/rpm/).

## Build, Test, and Development Commands

```bash
# Build debug version
make build          # or: cargo build

# Run with logging enabled
make run           # or: RUST_LOG=info cargo run

# Build optimized release binary
make release       # or: cargo build --release

# Run all quality checks
make check         # Runs fmt, clippy, and test

# Format code
make fmt           # or: cargo fmt

# Lint with Clippy
make clippy        # or: cargo clippy -- -D warnings

# Run tests
make test          # or: cargo test

# Clean build artifacts
make clean         # or: cargo clean
```

The release binary is generated at [target/release/print-layout](target/release/print-layout).

## Coding Style & Naming Conventions

- **Indentation**: 4 spaces (standard Rust convention)
- **Formatting**: Use `cargo fmt` with default rustfmt settings
- **Naming**: 
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types (structs, enums, traits)
  - `SCREAMING_SNAKE_CASE` for constants
- **Linting**: All code must pass `cargo clippy -- -D warnings` with zero warnings
- **Documentation**: Public APIs should have doc comments (`///`)

## Testing Guidelines

The project uses Rust's built-in testing framework (`#[cfg(test)]` modules). Run tests with `cargo test` or `make test`. While the project currently has limited unit tests, any new features should include appropriate test coverage for:

- Data structure serialization/deserialization (layouts, configurations)
- Layout calculations and image transformations
- Configuration validation logic

## Commit & Pull Request Guidelines

**Commit Message Format**: Based on recent history, commits follow these patterns:

- **Feature additions**: Descriptive messages starting with action verbs  
  _Example_: `Add comprehensive GUI rework documentation`
- **Version releases**: Include version number and key changes  
  _Example_: `Release v0.2.1: CUPS printer capabilities integration`
- **Bug fixes**: Clear description of what was fixed  
  _Example_: `Fix RPM spec Source0 URL for COPR builds`
- **Refactoring**: Note what was changed and why  
  _Example_: `Replace GTK3 with xdg-portal for file dialogs, fix UI icons`
- **Documentation**: Prefix with `docs:` for documentation-only changes  
  _Example_: `docs: Update project_plan.md with completion status`

**Pull Requests**:
- Ensure code passes `make check` (formatting, linting, tests)
- Include clear descriptions of changes and motivation
- Link related issues if applicable
- For UI changes, include screenshots or screen recordings
- Update relevant documentation files (README.md, USAGE.md, CHANGELOG.md)

## Architecture Overview

The application uses Iced's Elm-like architecture with unidirectional data flow:

1. **State** (`PrintLayout` struct in main.rs) holds all application state
2. **Messages** (`Message` enum) represent user actions and async events
3. **Update** function processes messages and returns new state + tasks
4. **View** function renders UI based on current state

Key design patterns:
- Image caching systems to optimize rendering performance
- Separate canvas widget for drag-drop and resize interactions
- CUPS integration via direct IPP communication
- Auto-save with backup rotation system
