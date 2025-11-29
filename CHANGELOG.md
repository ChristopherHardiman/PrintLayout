# Changelog

All notable changes to Print Layout will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-11-28

### Added

#### Image Management
- Add multiple images to layout (PNG, JPEG, GIF, BMP, WebP supported)
- Drag-and-drop positioning on canvas
- Drag-to-resize with 8 handles (4 corners + 4 edges)
- Visual selection highlighting with blue border
- Thumbnail panel for quick image selection and navigation

#### Image Manipulation
- Rotate images 90° clockwise and counter-clockwise
- Flip images horizontally and vertically
- Resize images with width/height inputs
- Aspect ratio lock option during resize
- Opacity control (0-100%) with live preview
- All transforms preserved in print output

#### Page Configuration
- Standard paper sizes: A-series (A0-A10), B-series (B0-B10)
- North American sizes: Letter, Legal, Tabloid, Ledger
- Photo paper sizes: 3.5×5", 4×6", 5×5", 5×7", 7×10", 8×10", 10×12", 11×17", 12×12", 13×19"
- Panorama and Custom Large paper support
- Paper types: Plain, Super High Gloss, Glossy, Semi-Gloss, Matte, Fine Art
- Custom margin controls (Top, Bottom, Left, Right in mm)
- Portrait/Landscape orientation toggle
- Borderless printing option

#### Printing Integration
- CUPS printer discovery and selection
- High-resolution rendering (300 DPI)
- Print quality presets: Highest, High, Standard, Draft
- Color mode selection: Use ICC Profile, Driver Matching, No Color Correction, Black and White
- Full transform support (rotation, flip, opacity) applied to print output
- Print job status feedback

#### Project Management
- Save layouts to `.pxl` format (JSON-based)
- Load previously saved layouts
- Auto-save every 30 seconds when modified
- Auto-save recovery dialog on startup
- Automatic backup system (keeps 5 most recent backups)
- Recent files menu with quick access (up to 10 files)
- Dirty state tracking with (*) indicator in window title
- Last print settings restoration

#### User Interface
- Canon Professional Print & Layout inspired design
- Tabbed settings panel (Print Settings, Layout, Color, Image Tools)
- Tools toolbar with zoom controls and orientation toggle
- Horizontal scrolling thumbnails area
- Status display for printer selection
- Modern Iced 0.13 canvas with image rendering

#### Configuration
- Persistent user preferences
- Default margin settings
- Last used printer memory
- Zoom level persistence
- Recent files tracking

### Technical Details
- Built with Rust and Iced 0.13 GUI toolkit
- Transform-based image caching for performance
- Source image caching to reduce disk I/O
- Optimized drag operations for smooth interaction
- XDG-compliant configuration directories

### Known Limitations
- Single page layouts only (multi-page planned for future)
- No PDF export (planned for future)
- No undo/redo system (planned for future)
- RAW image format not supported
- HEIC/HEIF format not supported

## [Unreleased]

### Planned Features
- Multi-page layout support
- PDF export
- Undo/redo system
- Template system
- Advanced color management with ICC profiles
- Keyboard shortcuts for all operations
- Drag-and-drop from file manager
- Batch printing
