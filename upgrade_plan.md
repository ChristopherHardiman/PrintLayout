# PrintLayout Upgrade Plan v0.2.0

**Target Version:** 0.2.0  
**Target Date:** Q1 2026  
**Focus Areas:** Multi-page support, PDF export, Undo/Redo system

---

## Executive Summary

This document outlines the implementation plan for three major features requested for PrintLayout v0.2.0. These features are interconnected and will significantly enhance the application's capabilities for professional print layout work.

### Priority Order
1. **Undo/Redo System** - Foundation for user experience (2 weeks)
2. **Multi-page Support** - Core feature enhancement (3 weeks)
3. **PDF Export** - Output flexibility (2 weeks)

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
Week 1-2:   Undo/Redo System
Week 3-5:   Multi-Page Support
Week 6-7:   PDF Export
Week 8:     Integration testing, bug fixes, documentation
```

### Total Estimated Effort: 8 weeks

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
