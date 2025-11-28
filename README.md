# PrintLayout

A lightweight, cross-desktop GUI application for creating professional page layouts with multiple images. Built in Rust with Iced, it supports standard paper sizes, custom margins, and direct printer integration via CUPS.

## Features

### Image Management
- Add multiple images (PNG, JPEG, GIF, BMP, WebP)
- Drag-and-drop positioning on canvas
- **Drag-to-resize** with 8 handles (corners and edges)
- Selection with visual highlighting

### Image Manipulation
- **Rotate** 90° clockwise/counter-clockwise
- **Flip** horizontal and vertical
- **Resize** with aspect ratio lock
- **Opacity** control (0-100%)

### Page Configuration
- Standard paper sizes (A-series, B-series, Letter, Legal, Tabloid, Ledger)
- Photo paper sizes (3.5×5" through 13×19")
- Custom margin controls
- Portrait/Landscape orientation

### Printing
- CUPS printer discovery and selection
- High-resolution rendering at 300 DPI
- Full transform support (rotation, flip, opacity) in print output

### Project Management
- Save/Load layouts (.pxl format)
- Auto-save every 30 seconds
- Automatic backup system
- Configuration persistence

## Building

```bash
cargo build --release
```

## Running

```bash
./target/release/print_layout
```

## Requirements

- Linux with X11 or Wayland
- CUPS installed and running (for printing)
- Rust 1.70+ (for building from source)

## License

Apache 2.0
