# PrintLayout Upgrade Plan v0.2.0

**Target Version:** 0.2.0  
**Target Date:** Q1 2026  
**Focus Areas:** Iced 0.14 upgrade, Undo/Redo system, Multi-page support, PDF export

---

## Executive Summary

This document outlines the Iced 0.14 upgrade and three major features planned for PrintLayout. The Iced upgrade is a prerequisite for feature work; the features will significantly enhance the application's capabilities for professional print layout work.

### Priority Order
1. **Iced 0.14 Upgrade** - Update the GUI framework and Rust toolchain before feature work
2. **Undo/Redo System** - Foundation for user experience (2 weeks)
3. **Multi-page Support** - Core feature enhancement (3 weeks)
4. **PDF Export** - Output flexibility (2 weeks)

---

## Platform Upgrade: Iced 0.14

### Overview

Upgrade the application from Iced 0.13 to 0.14 before starting the planned features. A trial migration in a throwaway copy compiled successfully after the API changes below; the application still needs runtime and manual verification.

Iced 0.14 brings concurrent image decoding and uploading, a reactive renderer, headless and end-to-end testing support, and a unified keyboard subscription API. Keep the project edition at 2021.

### Implementation Steps

#### Step 1: Establish a Baseline
- [x] Create an upgrade branch, such as `iced-0.14`.
- [x] Record the release binary size: 23,198,248 bytes (23.2 MB) on Iced 0.13. Baseline `cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test` (2 tests) all passed.
- [ ] Run a manual Iced 0.13 smoke test: add images, drag and resize them, save/load a layout, and run a test print. (Skipped; compare against 0.14 behavior by hand in Step 4.)

#### Step 2: Update the Toolchain and Dependency
- [x] Set the project's `rust-version` to at least 1.88, required by Iced 0.14 dependencies.
- [x] Update the RPM `BuildRequires` and Rust-version references in GEMINI.md. README.md has no Rust version reference.
- [ ] Confirm Fedora/COPR and GitHub Actions build environments provide Rust 1.88 or newer. CI uses `dtolnay/rust-toolchain@stable` and the RPM workflow uses `fedora:latest`, so both should qualify; the COPR chroots are still to be checked.
- [x] Change the Iced dependency in Cargo.toml to version `0.14` and update the lockfile (`Cargo.lock` is untracked, resolved to iced 0.14.0).

#### Step 3: Migrate Iced APIs (done)

| Area | Iced 0.13 | Iced 0.14 |
|---|---|---|
| Application startup | `application(title, update, view).run_with(new)` | `application(new, update, view).title(...).run()` |
| Rules | `horizontal_rule(1)`, `vertical_rule(1)` | `rule::horizontal(1)`, `rule::vertical(1)` |
| Spacing | `Space::with_width(w)`, `Space::with_height(h)` | `Space::new().width(w)`, `Space::new().height(h)` |
| Checkboxes | `checkbox("label", checked)` | `checkbox(checked).label("label")` |
| Progress bar sizing | `.width(..).height(..)` | `.length(..).girth(..)` |
| Default theme | `Theme::default()` | Use `Theme::Light` or implement system-theme behavior |
| Canvas events | `canvas::Event`; return `(Status, Option<Msg>)` | `&iced::Event`; return `Option<canvas::Action<Msg>>` |
| Canvas event matching | `canvas::Event::Mouse(..)` | `iced::Event::Mouse(..)` |
| Canvas response | `(Status::Captured, Some(msg))` / `(Status::Ignored, None)` | `Some(canvas::Action::publish(msg).and_capture())` / `None` |

#### Step 4: Verify
- [x] Run `cargo fmt --check`, `cargo build`, `cargo test`, and `cargo clippy -- -D warnings`; all pass, matching the baseline.
- [x] Build with `cargo build --release` and compare binary size with the baseline: 23,286,536 bytes (+0.4%). Link memory was not measured.
- [x] Launch smoke test on Wayland: the window and wgpu surface are created, printers are discovered, and there is no panic.
- [ ] On Wayland and X11, manually check image loading, canvas drag/resize/selection, transforms, settings controls, dialogs, save/load, auto-save, recovery, and printing.
- [ ] Confirm file dialogs continue to work through xdg-portal.

#### Step 5: Ship
- [ ] Update CHANGELOG.md and status.md with the upgrade result and any remaining limitations.
- [ ] Bump the application version as appropriate, then merge and tag after CI and RPM workflows pass.

### Optional Follow-ups
- Implement the documented keyboard shortcuts using `keyboard::listen`.
- Evaluate Iced 0.14 widgets (`float`, `pin`, `grid`, `table`) and `Column::wrap` for future UI work.
- Add headless end-to-end tests where useful.

### Risks
- Rust 1.88 may not be available in all supported RPM build environments.
- Iced 0.14 changes layout rules, quad rendering, and theme palette generation; spacing and colors may shift.
- The canvas event API rewrite needs careful manual testing for drag and resize behavior.
- Iced is pre-1.0, so future releases may introduce more breaking API changes.

---

## Feature 1: Undo/Redo System

### Overview
Implement a command-based undo/redo system that tracks all user actions and allows reversing them.

### Current State Analysis

The current codebase has these relevant structures:
- `Layout` struct contains `Page` and `Vec<PlacedImage>`
- `PlacedImage` has properties: position, size, rotation, flip, opacity
- State changes happen through direct mutation in `Message` handlers
- No history tracking exists

### Design Approach: Command Pattern

```rust
// src/history.rs (new module)

/// Represents a reversible action
#[derive(Debug, Clone)]
pub enum Command {
    // Image commands
    AddImage { image: PlacedImage },
    RemoveImage { image: PlacedImage, index: usize },
    MoveImage { id: String, from: (f32, f32), to: (f32, f32) },
    ResizeImage { id: String, from: (f32, f32), to: (f32, f32) },
    RotateImage { id: String, from: f32, to: f32 },
    FlipImage { id: String, horizontal: bool, vertical: bool },
    SetOpacity { id: String, from: f32, to: f32 },
    
    // Page commands
    SetPaperSize { from: PaperSize, to: PaperSize },
    SetMargins { from: (f32, f32, f32, f32), to: (f32, f32, f32, f32) },
    SetOrientation { from: Orientation, to: Orientation },
    ToggleBorderless { from: bool, to: bool },
    
    // Compound commands (for grouping)
    Batch(Vec<Command>),
}

impl Command {
    /// Execute the command (redo)
    pub fn execute(&self, layout: &mut Layout) { ... }
    
    /// Reverse the command (undo)
    pub fn undo(&self, layout: &mut Layout) { ... }
    
    /// Get human-readable description for UI
    pub fn description(&self) -> &str { ... }
}

/// Manages command history for undo/redo
pub struct History {
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    max_history: usize,
}

impl History {
    pub fn new(max_history: usize) -> Self { ... }
    
    pub fn push(&mut self, cmd: Command) {
        self.undo_stack.push(cmd);
        self.redo_stack.clear(); // Clear redo on new action
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }
    
    pub fn undo(&mut self, layout: &mut Layout) -> bool { ... }
    pub fn redo(&mut self, layout: &mut Layout) -> bool { ... }
    
    pub fn can_undo(&self) -> bool { !self.undo_stack.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo_stack.is_empty() }
}
```

### Implementation Steps

#### Step 1: Create History Module (3 days)
- [ ] Create `src/history.rs` with `Command` enum and `History` struct
- [ ] Implement `execute()` and `undo()` for all command types
- [ ] Add unit tests for each command type
- [ ] Add `history` module to `lib.rs`

#### Step 2: Integrate with Main Application (3 days)
- [ ] Add `history: History` field to `PrintLayout` struct
- [ ] Modify `Message` handlers to create commands instead of direct mutation
- [ ] Add `Message::Undo` and `Message::Redo` variants
- [ ] Track drag operations as single commands (batch move deltas)

#### Step 3: Add UI Controls (2 days)
- [ ] Add Undo/Redo buttons to toolbar
- [ ] Implement keyboard shortcuts: `Ctrl+Z` (undo), `Ctrl+Shift+Z` / `Ctrl+Y` (redo)
- [ ] Show undo/redo state in button enabled/disabled status
- [ ] Optional: Add Edit menu with undo history preview

#### Step 4: Serialization Support (2 days)
- [ ] Add `#[derive(Serialize, Deserialize)]` to `Command` enum
- [ ] Include history in auto-save (optional, may skip for performance)
- [ ] Clear history on file load (new document = clean slate)

### Messages to Add

```rust
pub enum Message {
    // ... existing messages ...
    
    // Undo/Redo
    Undo,
    Redo,
}
```

### Testing Plan
- Test undo/redo for each individual command type
- Test command batching (e.g., drag = many small moves → single undo)
- Test history limit enforcement
- Test redo stack clearing on new action
- Test keyboard shortcuts

### Estimated Effort: 2 weeks

---

## Feature 2: Multi-Page Support

### Overview
Allow users to create documents with multiple pages, each with its own images and settings.

### Current State Analysis

Current structure (single page):
```rust
pub struct Layout {
    pub page: Page,           // Single page
    pub images: Vec<PlacedImage>,
    pub selected_image_id: Option<String>,
}
```

### Design Approach

#### New Data Structures

```rust
// src/layout.rs modifications

/// A single page with its images
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageContent {
    pub page: Page,
    pub images: Vec<PlacedImage>,
}

/// Multi-page document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub pages: Vec<PageContent>,
    pub current_page_index: usize,
    pub selected_image_id: Option<String>,
    
    // Document-level settings
    pub name: String,
    pub default_page_settings: Page,  // Template for new pages
}

impl Document {
    pub fn new() -> Self {
        Self {
            pages: vec![PageContent::new(Page::default())],
            current_page_index: 0,
            selected_image_id: None,
            name: "Untitled".to_string(),
            default_page_settings: Page::default(),
        }
    }
    
    pub fn current_page(&self) -> &PageContent { ... }
    pub fn current_page_mut(&mut self) -> &mut PageContent { ... }
    pub fn add_page(&mut self) -> usize { ... }
    pub fn remove_page(&mut self, index: usize) -> Option<PageContent> { ... }
    pub fn duplicate_page(&mut self, index: usize) { ... }
    pub fn move_page(&mut self, from: usize, to: usize) { ... }
    pub fn page_count(&self) -> usize { ... }
}

// Backwards compatibility alias
pub type Layout = Document;
```

### Implementation Steps

#### Step 1: Refactor Data Structures (3 days)
- [ ] Create `PageContent` struct
- [ ] Create `Document` struct to replace `Layout`
- [ ] Add page navigation methods
- [ ] Update serialization for backwards compatibility with v0.1 `.pxl` files

#### Step 2: Update Canvas Widget (3 days)
- [ ] Modify `LayoutCanvas` to accept current page reference
- [ ] Add page indicator overlay (e.g., "Page 1 of 3")
- [ ] Handle page-specific image selection

#### Step 3: Add Page Navigation UI (4 days)
- [ ] Add page thumbnail strip (horizontal, below canvas or vertical on left)
- [ ] Implement page thumbnails with mini-preview
- [ ] Add "Previous Page" / "Next Page" buttons
- [ ] Add page context menu (Add, Delete, Duplicate, Move)
- [ ] Keyboard shortcuts: `Page Up`, `Page Down`, `Ctrl+Page Up/Down`

#### Step 4: Update Page Management (3 days)
- [ ] "Add Page" button in toolbar or page panel
- [ ] "Delete Page" with confirmation (can't delete last page)
- [ ] "Duplicate Page" (copies all images and settings)
- [ ] Drag-and-drop page reordering in thumbnail strip

#### Step 5: Update File Operations (2 days)
- [ ] Update `ProjectLayout` to use new `Document` struct
- [ ] Add migration code for loading old single-page `.pxl` files
- [ ] Update auto-save to handle multi-page documents

#### Step 6: Update Printing (3 days)
- [ ] Print all pages sequentially
- [ ] Add "Print Range" option (All, Current, Custom range)
- [ ] Update print job to handle multiple pages
- [ ] Add page break handling in print output

#### Step 7: Integrate with Undo/Redo (2 days)
- [ ] Add page-level commands to history:
  - `AddPage`, `RemovePage`, `DuplicatePage`, `MovePage`
- [ ] Ensure page navigation doesn't create history entries
- [ ] Handle cross-page image operations (future: move image between pages)

### Messages to Add

```rust
pub enum Message {
    // ... existing messages ...
    
    // Page management
    AddPage,
    DeletePage(usize),
    DuplicatePage(usize),
    MovePage { from: usize, to: usize },
    GoToPage(usize),
    NextPage,
    PreviousPage,
    
    // Page thumbnails
    PageThumbnailClicked(usize),
    PageThumbnailDragStart(usize),
    PageThumbnailDragEnd(usize),
}
```

### UI Mockup

```
┌────────────────────────────────────────────────────────────────────┐
│  [+Add] [Open] [Save]  │  [Undo] [Redo]  │  Zoom: [+][-][100%]   │
├────────────────────────────────────────────────────────────────────┤
│        │                                           │              │
│ Pages  │              Canvas                       │  Settings   │
│ ┌────┐ │                                           │  Panel      │
│ │ 1  │ │    ┌─────────────────────┐               │              │
│ └────┘ │    │                     │               │  [Print]    │
│ ┌────┐ │    │   Page 1 of 3       │               │  [Layout]   │
│ │ 2  │◄│    │                     │               │  [Color]    │
│ └────┘ │    │                     │               │  [Image]    │
│ ┌────┐ │    └─────────────────────┘               │              │
│ │ 3  │ │                                           │              │
│ └────┘ │    [< Prev]  Page 2  [Next >]            │              │
│ [+Add] │                                           │              │
├────────┴───────────────────────────────────────────┴──────────────┤
│  [img1] [img2] [img3] [img4]  (thumbnails for current page)       │
└────────────────────────────────────────────────────────────────────┘
```

### Backwards Compatibility

```rust
impl ProjectLayout {
    /// Load v0.1 single-page format and convert to multi-page
    fn migrate_v01(old: V01ProjectLayout) -> Self {
        Self {
            version: "0.2.0".to_string(),
            document: Document {
                pages: vec![PageContent {
                    page: old.layout.page,
                    images: old.layout.images,
                }],
                current_page_index: 0,
                selected_image_id: old.layout.selected_image_id,
                ..Default::default()
            },
            ..
        }
    }
}
```

### Estimated Effort: 3 weeks

---

## Feature 3: PDF Export

### Overview
Allow users to export their layouts as PDF files for sharing, archiving, or printing on other systems.

### Dependencies
- Recommended crate: `printpdf` (pure Rust, no external dependencies)
- Alternative: `lopdf` (lower level), `cairo-rs` (requires system libs)

### Design Approach

```rust
// src/pdf_export.rs (new module)

use printpdf::*;
use image::DynamicImage;

pub struct PdfExportOptions {
    pub dpi: u32,                    // Default: 300
    pub include_margins: bool,       // Show margin lines
    pub embed_images: bool,          // Embed vs reference
    pub compression: PdfCompression,
    pub metadata: PdfMetadata,
}

pub struct PdfMetadata {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub keywords: Vec<String>,
    pub creator: String,  // "PrintLayout v0.2.0"
}

pub enum PdfCompression {
    None,
    Fast,
    Best,
}

/// Export a document to PDF
pub fn export_to_pdf(
    document: &Document,
    path: &PathBuf,
    options: &PdfExportOptions,
    image_cache: &ImageCache,
) -> Result<(), PdfExportError> {
    let (doc, page_indices) = create_pdf_document(document, options)?;
    
    for (page_idx, page_content) in document.pages.iter().enumerate() {
        render_page_to_pdf(&doc, page_indices[page_idx], page_content, options, image_cache)?;
    }
    
    doc.save(&mut BufWriter::new(File::create(path)?))?;
    Ok(())
}
```

### Implementation Steps

#### Step 1: Add PDF Dependency (1 day)
- [ ] Add `printpdf = "0.7"` to Cargo.toml
- [ ] Create `src/pdf_export.rs` module
- [ ] Add basic PDF generation test

#### Step 2: Implement Core Export (4 days)
- [ ] Create PDF document with correct page dimensions
- [ ] Convert mm to PDF points (1 point = 1/72 inch)
- [ ] Implement image embedding with proper scaling
- [ ] Apply image transforms (rotation, flip, opacity)
- [ ] Handle multi-page documents

#### Step 3: Add Export Options UI (2 days)
- [ ] Create "Export to PDF" dialog
- [ ] Add DPI selection (72, 150, 300, 600)
- [ ] Add page range selection (All, Current, Custom)
- [ ] Add metadata input fields (optional)
- [ ] Show estimated file size

#### Step 4: Integrate with Application (2 days)
- [ ] Add `Message::ExportPdf` and `Message::PdfExportComplete`
- [ ] Add "Export PDF" button to toolbar or File menu
- [ ] Add keyboard shortcut: `Ctrl+Shift+E`
- [ ] Add progress indicator for large exports

#### Step 5: Error Handling (1 day)
- [ ] Handle missing image files gracefully
- [ ] Show export errors in UI
- [ ] Add logging for debugging

### PDF Coordinate System

```
PDF uses points (1/72 inch) with origin at bottom-left.
PrintLayout uses mm with origin at top-left.

Conversion:
  pdf_x = mm_x * (72 / 25.4)
  pdf_y = (page_height_mm - mm_y) * (72 / 25.4)  // Flip Y axis
```

### Messages to Add

```rust
pub enum Message {
    // ... existing messages ...
    
    // PDF Export
    ExportPdfClicked,
    PdfExportPathSelected(Option<PathBuf>),
    PdfExportComplete(Result<PathBuf, String>),
    PdfExportOptionsChanged(PdfExportOptions),
}
```

### Testing Plan
- Export single-page layout, verify in PDF reader
- Export multi-page layout, verify page order
- Test image transforms (rotation, flip) in PDF output
- Test with missing image files
- Verify PDF metadata
- Test large images (memory usage)
- Compare PDF output with print output (should match)

### Estimated Effort: 2 weeks

---

## Integration Timeline

```
Before Week 1: Iced 0.14 upgrade and verification
Week 1-2:      Undo/Redo System
Week 3-5:      Multi-Page Support
Week 6-7:      PDF Export
Week 8:        Integration testing, bug fixes, documentation
```

### Feature Work Estimate: 8 weeks, after the Iced upgrade

---

## Technical Considerations

### Memory Management
- Multi-page documents will use more memory
- Consider lazy-loading page images when not visible
- Implement page thumbnail caching

### Performance
- History should have configurable max size (default: 100 commands)
- Large command batches should be compressed
- PDF export should show progress for large documents

### File Format
- Version `.pxl` files as "0.2.0" for new format
- Maintain backwards compatibility with "0.1.0" files
- Consider adding format migration on load

### Testing Strategy
- Unit tests for Command execute/undo
- Unit tests for Document page operations
- Integration tests for PDF export
- Manual testing for UI flows

---

## Cargo.toml Changes

```toml
[dependencies]
# ... existing dependencies ...

# PDF Export
printpdf = "0.7"
```

---

## New Modules Summary

| Module | Purpose | Lines (est.) |
|--------|---------|--------------|
| `src/history.rs` | Undo/redo command system | ~400 |
| `src/pdf_export.rs` | PDF export functionality | ~300 |
| `src/layout.rs` | Modified for multi-page | +200 |
| `src/main.rs` | UI updates | +400 |

---

## Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| History memory usage | Medium | Low | Configurable limit, LRU eviction |
| PDF image quality | Low | Medium | Test with various DPI settings |
| Multi-page performance | Medium | Medium | Lazy loading, thumbnail caching |
| Backwards compatibility | Low | High | Thorough migration testing |
| UI complexity | Medium | Medium | Iterative design, user feedback |

---

## Success Criteria

1. **Undo/Redo**: User can undo/redo at least 50 operations reliably
2. **Multi-page**: User can create 10+ page documents without performance issues
3. **PDF Export**: Exported PDF matches canvas preview at 300 DPI
4. **Backwards Compatibility**: All v0.1.0 `.pxl` files load correctly

---

## Future Enhancements (Post v0.2.0)

- Undo history visualization (timeline view)
- Page templates (apply to multiple pages)
- PDF/A archival format support
- Move images between pages via drag-and-drop
- Page numbering overlays
- Master pages (shared elements across pages)
