# Project Plan: Print Layout Application

This document outlines the detailed steps to implement the Print Layout application using Rust and the Iced GUI toolkit.

---

## Progress Summary

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1 | ✅ Complete | Project Setup & Basic Window |
| Phase 2 | ✅ Complete | Core Layout Engine & Canvas |
| Phase 3 | ✅ Complete | UI Controls & File Management |
| Phase 4 | ✅ Complete | Printing Integration |
| Phase 5 | 🔄 Partial | Persistence & State Management (basic save/load implemented) |
| Phase 6 | ⬜ Not Started | Packaging & Final Touches |

### Recent Updates (November 2025)

- **Iced 0.13 Upgrade**: Upgraded from Iced 0.12 to 0.13 for improved canvas image rendering support
- **Image Rendering**: Implemented actual image rendering on canvas using `frame.draw_image()` API
- **Canvas Widget**: Uses `RefCell` for interior mutability to enable image caching during draw operations
- **Image Cache**: Implemented `ImageCache` with `iced::widget::image::Handle` for efficient image loading

---

### Phase 1: Project Setup & Basic Window ✅ COMPLETE

**Goal:** Initialize the project and get a basic, empty window to appear on the screen.

1.  **Initialize Cargo Project:**
    *   Create a new binary-based Rust project.
    *   This will create the basic directory structure and `Cargo.toml`.
    ```bash
    cargo new --bin print_layout
    cd print_layout
    ```
    *   Verify the project structure is created with `src/main.rs` and `Cargo.toml`.

2.  **Configure Cargo.toml with Full Dependencies:**
    *   Edit `Cargo.toml` to include all Phase 1 dependencies and metadata.
    ```toml
    [package]
    name = "print_layout"
    version = "0.1.0"
    edition = "2021"
    authors = ["Your Name <your.email@example.com>"]
    description = "Lightweight cross-desktop GUI for creating print layouts"
    license = "Apache-2.0"
    repository = "https://github.com/ChristopherHardiman/PrintLayout"
    
    [dependencies]
    iced = { version = "0.12", features = ["canvas", "tokio", "debug"] }
    image = "0.25"
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    rfd = "0.14"
    tokio = { version = "1", features = ["full"] }
    log = "0.4"
    env_logger = "0.11"
    directories = "5.0"  # XDG directory support
    uuid = { version = "1.0", features = ["v4"] }
    chrono = { version = "0.4", features = ["serde"] }
    ```
    *   Set a reasonable initial window size in the application settings (e.g., 1200x800).
    *   Add a version constant that can be updated for future releases.
    *   **Dependency Strategy:** Pin major versions to avoid breaking changes. Document fallback strategies for critical dependencies like CUPS integration.

3.  **Create Module Structure:**
    *   Create `src/main.rs` as the entry point.
    *   Create `src/lib.rs` to organize modules.
    *   Plan module hierarchy (to be filled in later phases):
        - `src/layout.rs` - Page and image data structures
        - `src/canvas.rs` - Canvas widget implementation
        - `src/ui.rs` - UI controls and layouts
        - `src/printing.rs` - CUPS integration
        - `src/state.rs` - Application state management
        - `src/config.rs` - Configuration and preferences
    *   Start with stubs for these modules that can be filled in during subsequent phases.

4.  **Create the Main Application State:**
    *   In `src/main.rs`, define an enum `Message` with variants for all user interactions (to be expanded in later phases):
        ```rust
        #[derive(Debug, Clone)]
        pub enum Message {
            WindowClosed,
            // More messages to be added
        }
        ```
    *   Define a struct `PrintLayout` containing:
        - A `page` field (to be defined in Phase 2)
        - A `selected_image` field (Option tracking which image is selected)
        - A `clipboard` for temporary state during operations
        - A `window_size` field for responsive rendering
    *   Implement the `iced::Application` trait with:
        - `::new()` - Initialize with default values (empty layout, default A4 page, 1-inch margins)
        - `::title()` - Return "Print Layout"
        - `::update()` - Handle messages (initially just exit on window close)
        - `::view()` - Return a basic layout

5.  **Implement the Basic View:**
    *   In the `view` method, create a layout structure:
        - A top menu bar (placeholder for File, Edit, Help menus - to be implemented later)
        - A main content area showing "Print Layout" and "Ready"
        - A status bar at the bottom showing application version
    *   Use `iced::widget::Column` for vertical layout with:
        - `iced::widget::Row` for horizontal sections
        - `iced::widget::Text` for labels and information
        - Basic spacing and padding for visual hierarchy
    *   Ensure responsive sizing so the window can be resized without crashes.

6.  **Set Up Window Configuration:**
    *   Define window settings struct to manage:
        - Initial window size (1200x800)
        - Minimum window size (800x600) to prevent UI breakage
        - Window title and application identifier
        - Icon (create a simple placeholder icon in SVG or image format for future use)
    *   Configure settings in `iced::Application::view()` using window settings returned from the application.

7.  **Test the Basic Application:**
    *   Run with `cargo run` and verify:
        - Window opens with the correct title
        - Window displays the basic layout without crashes
        - Window can be resized without layout breaking
        - Window closes properly when the close button is clicked
        - Console shows no warnings or errors during compilation
    *   Test on both X11 and Wayland environments (if possible) to ensure cross-desktop compatibility.

8.  **Set Up Development Tooling:**
    *   Create a `.cargo/config.toml` file for build optimization:
        ```toml
        [profile.release]
        opt-level = 3
        lto = true
        codegen-units = 1
        ```
    *   Add `.gitignore` entries for Rust build artifacts.
    *   Create `Makefile` or shell scripts for common tasks (build, run, test, clean).

9.  **Set Up CI/CD Pipeline:**
    *   Create `.github/workflows/ci.yml` for GitHub Actions:
        - Run `cargo fmt --check` on every PR
        - Run `cargo clippy` for linting
        - Run `cargo test` for all tests
        - Build release binaries for x86_64 and ARM64
    *   Configure automated releases on tag push
    *   Set up cross-compilation targets for different Linux architectures

---

### Phase 2: Core Layout Engine & Canvas ✅ COMPLETE

**Goal:** Create a canvas where images can be programmatically placed, moved, and resized.

1.  **Define Core Data Structures in `src/layout.rs`:**
    *   Create `struct Page` containing:
        - `width_mm` and `height_mm` - Physical dimensions
        - `margin_top`, `margin_bottom`, `margin_left`, `margin_right` - All as floats in mm
        - `orientation` - Enum for Portrait/Landscape
        - Helper method `printable_area()` to calculate usable space
        - Helper method `to_pixels(dpi: u32)` for rendering calculations
    *   Create `struct PlacedImage` containing:
        - `id` - Unique identifier (UUID or auto-incrementing number)
        - `source_path` - Path to the image file
        - `position_x`, `position_y` - Position on page in mm
        - `width_mm`, `height_mm` - Dimensions in mm
        - `rotation_degrees` - 0-360 degrees for rotation
        - `z_order` - Layer ordering for overlapping images
        - `is_selected` - Current selection state
        - `is_visible` - Visibility toggle for layers panel
        - `is_locked` - Prevent accidental modification
        - `original_width_px`, `original_height_px` - Native image dimensions for DPI calculations
        - `border_width_mm` - Optional border/frame width
        - `border_color` - Optional border color (RGBA)
        - Helper methods: `bounds_mm()`, `contains_point(x, y)` for hit testing
        - Helper method: `effective_dpi()` - Calculate print DPI based on current size vs original pixels
    *   Create `struct Layout` containing:
        - `page` - The Page configuration
        - `images` - Vec of PlacedImage
        - `dpi` - Dots per inch for rendering (default 300 for printing)
        - Methods: `add_image()`, `remove_image()`, `get_image_mut()`, `find_image_at_point()`
    *   Add serialization derives to all structures for Phase 5 integration.

2.  **Implement Standard Paper Sizes in `src/layout.rs`:**
    *   Create `enum PaperSize` with variants: A0-A10, Letter, Legal, Tabloid, Ledger, B-series (B0-B10), and Custom
    *   Create `enum PaperType` with variants: MattePhoto, GlossPhoto, PhotoPaper, PrinterPaper, Satin, Canvas, RicePaper, Cardstock, Transparency
    *   Implement `trait` or methods to convert PaperSize to (width_mm, height_mm)
    *   Implement helper function `paper_size_to_dimensions(size: PaperSize) -> (f32, f32)`
    *   Document all standard sizes with their exact dimensions for accuracy:
        - A4: 210 × 297 mm
        - Letter: 215.9 × 279.4 mm (8.5 × 11 in)
        - Legal: 215.9 × 355.6 mm (8.5 × 14 in)
        - etc.
    *   **Regional Defaults:** Detect system locale and default to appropriate paper size (Letter for US/Canada, A4 for most other regions).

3.  **Create Image Cache in `src/canvas.rs`:**
    *   Implement an `ImageCache` struct that stores loaded `iced::widget::canvas::Image` objects
    *   Cache uses HashMap<Path, ImageHandle> to avoid reloading the same file
    *   Add cache invalidation method for when images are deleted or replaced
    *   Implement bounds checking and error handling for missing/corrupted image files
    *   Load images in background using tokio tasks to prevent UI freezing (important for Phase 4 integration)
    *   **Supported Image Formats:**
        - Core: PNG, JPEG, GIF, BMP, WebP
        - Extended: TIFF (common for print workflows)
        - Future consideration: HEIC/HEIF (requires additional library)
        - Note: RAW format support deferred to future version (requires specialized libraries)
    *   **Memory Management:**
        - Set configurable cache size limit (default 500MB)
        - Implement LRU (Least Recently Used) eviction policy
        - Store image metadata separately from pixel data
        - Unload full resolution when not actively editing

4.  **Implement a Custom Canvas Widget in `src/canvas.rs`:**
    *   Create `struct LayoutCanvas` implementing `iced::widget::canvas::Program`
    *   Implement `draw()` method that:
        - Draws the page background with clear boundaries (using `Path` and `Stroke`)
        - Draws margin lines (dashed lines) to show safe printing area
        - Draws each PlacedImage with proper transformations (scale, rotation)
        - Draws selection indicators (bounding box with corner handles) for selected image
        - Draws grid overlay (optional, toggleable for alignment aid)
        - Uses proper color scheme (white page, light gray margins, handle indicators in accent color)
        - **DPI Warning Indicator:** Draw red/orange border on images that will print below 150 DPI at current size
        - **Alignment Guides:** Draw snap lines when dragging near other images, page center, or margins
    *   Implement proper coordinate transformation from mm to pixels based on zoom level
    *   Add zoom support (Ctrl+Scroll) to scale canvas view (10%-500% range)
    *   **Smart Snapping System:**
        - Snap to grid (configurable: 1mm, 5mm, 10mm)
        - Snap to other image edges and centers
        - Snap to page center lines (horizontal and vertical)
        - Snap to margin boundaries
        - Hold Alt to temporarily disable snapping

5.  **Handle Mouse Input on the Canvas:**
    *   Implement `on_event()` method to capture:
        - `Event::Mouse(mouse::Event::ButtonPressed(Button::Left))` - Start drag or select
        - `Event::Mouse(mouse::Event::CursorMoved{position})` - Update drag position
        - `Event::Mouse(mouse::Event::ButtonReleased(Button::Left))` - End drag
        - `Event::Mouse(mouse::Event::ScrolledVertical(delta))` - Zoom with Ctrl modifier
    *   Implement hit detection logic:
        - Check if click is on an image (detect which PlacedImage is under cursor)
        - Check if click is on resize handles (8 handles: corners and edges)
        - Check if click is on empty canvas (deselect current image)
    *   Implement drag modes:
        - `DragMode::MoveImage` - Translate selected image with constrained boundaries (keep within margins)
        - `DragMode::ResizeHandle(HandlePosition)` - Resize from specific handle with aspect ratio option
        - `DragMode::None` - No active drag
        - `DragMode::PanCanvas` - Middle-click or Space+drag to pan view
        - `DragMode::SelectionBox` - Click+drag on empty area to select multiple images
    *   Store drag state in a `drag_state` field with:
        - Current drag mode
        - Original position/size before drag started
        - Current cursor position
        - Offset from cursor to element center
    *   **Drag and Drop from File Manager:**
        - Accept dropped image files onto canvas
        - Parse `text/uri-list` MIME type for file paths
        - Place dropped image at cursor position
        - Support dropping multiple files at once (arrange in grid)
    *   **Multi-Selection Support:**
        - Ctrl+Click to add/remove images from selection
        - Shift+Click to select range (by z-order)
        - Selection box to select multiple images at once
        - Move/resize operations apply to all selected images

6.  **Produce and Handle Canvas Messages:**
    *   Define `CanvasMessage` enum:
        - `SelectImage(image_id)`
        - `SelectMultipleImages(Vec<image_id>)`
        - `DeselectImage`
        - `DeselectAll`
        - `MoveImage { image_id, delta_x, delta_y }`
        - `MoveSelectedImages { delta_x, delta_y }`
        - `ResizeImage { image_id, new_width, new_height }`
        - `RotateImage { image_id, angle_delta }`
        - `ChangeZOrder { image_id, direction: ZOrder }`
        - `ToggleImageVisibility(image_id)`
        - `ToggleImageLock(image_id)`
        - `AlignImages { alignment: Alignment }` - Left, Right, Top, Bottom, Center H, Center V
        - `DistributeImages { direction: Direction }` - Evenly space selected images
        - `FilesDropped(Vec<PathBuf>)`
    *   Forward these messages to main `Message` enum
    *   In `update()` function, handle each message by:
        - Validating the operation is allowed (check if image is locked)
        - Updating the layout state
        - Marking layout as modified (for dirty tracking)
        - Requesting redraw

7.  **Implement Keyboard Shortcuts in Canvas:**
    *   Support keyboard events for canvas efficiency:
        - `Delete` - Remove selected image(s)
        - `Ctrl+A` - Select all images
        - `Ctrl+C` - Copy selected image(s) (store in clipboard)
        - `Ctrl+V` - Paste copied image(s) with offset
        - `Ctrl+D` - Duplicate selected image(s) in place
        - `Arrow Keys` - Fine movement of selected image (1mm per press)
        - `Shift+Arrow Keys` - Faster movement (10mm per press)
        - `+/-` or `R/Shift+R` - Rotate selected image by 5 degrees
        - `Z` - Cycle through overlapping images at cursor
        - `F` - Fit image to page while maintaining aspect ratio
        - `L` - Toggle lock on selected image
        - `H` - Toggle visibility of selected image
        - `[` / `]` - Move selected image down/up in z-order
        - `Ctrl+[` / `Ctrl+]` - Send to back / Bring to front
        - `Escape` - Deselect all
        - `Space` (hold) - Temporarily enable canvas panning mode

8.  **Test Canvas Implementation:**
    *   Verify canvas renders correctly with:
        - Different page sizes
        - Multiple images at various positions
        - Zoom in/out maintains proper rendering
        - Drag and resize operations update state correctly
        - Hit detection accuracy for overlapping images
        - Keyboard shortcuts work as expected
    *   Test edge cases:
        - Images dragged outside margins
        - Very small images still selectable
        - Images at page boundaries

---

### Phase 3: UI Controls & File Management ✅ COMPLETE

**Goal:** Add UI elements to interact with the layout engine.

1.  **Create the Main UI Layout in `src/ui.rs`:**
    *   Structure the main view as:
        ```
        [Menu Bar: File | Edit | View | Help]
        [Toolbar: Add Image | Delete | Undo | Redo | Zoom Controls]
        [Left Panel: Properties Panel] [Center: Canvas] [Right Panel: Layers Panel]
        [Status Bar: Image Count | Page Info | Printer Status]
        ```
    *   Left panel (Properties) containing:
        - Paper size dropdown
        - Paper type dropdown
        - Margin controls (4 separate number inputs)
        - Orientation toggle (Portrait/Landscape)
        - Page preview miniature
    *   Right panel (Layers) containing:
        - List of all placed images
        - Visibility toggles (eye icon) per image
        - Layer ordering (up/down buttons)
        - Image preview thumbnail
        - Delete button for each layer
    *   Make panels collapsible to maximize canvas space

2.  **Implement "Add Image" Button and File Dialog:**
    *   Add button to toolbar with tooltip "Add Image to Layout (Ctrl+I)"
    *   Add keyboard shortcut `Ctrl+I` for Add Image (Ctrl+O reserved for Open Layout)
    *   On click, create `Message::AddImageClicked`
    *   In `update()` function, handle this message:
        - Use `rfd::AsyncFileDialog` to open non-blocking file picker
        - Filter for image files (.png, .jpg, .jpeg, .gif, .bmp, .webp, .tiff, .tif)
        - Allow multiple file selection
        - Launch picker in background task using `iced::Command::perform()`
        - Return `Message::ImageFilesSelected(Vec<path>)` when files are chosen
    *   Handle `Message::ImageFilesSelected(paths)`:
        - For each path:
            - Validate file exists and is readable
            - Attempt to load image metadata (dimensions, format)
            - Check file size (warn if > 50MB, reject if > 100MB)
            - Create new `PlacedImage` with default positioning:
                - Position: center of page (calculated from page dimensions), offset for multiple images
                - Size: Scale image to fit on page while maintaining aspect ratio (max 80% of printable area)
                - Rotation: 0 degrees
                - Z-order: highest (top layer)
            - Store original pixel dimensions for DPI calculations
            - Add to layout
        - Select the last added image (or all if multiple)
        - Cache the images for rendering
        - Show success or error notification with count
    *   **DPI Quality Check:**
        - Calculate effective DPI based on image pixels vs. placed size
        - Show warning if DPI < 150: "Image may appear pixelated when printed"
        - Show info if DPI > 300: "Image quality is excellent for printing"

3.  **Implement Image Deletion:**
    *   Add Delete button to toolbar and right-click context menu on canvas
    *   Keyboard shortcut `Delete` key
    *   On delete request:
        - Show confirmation dialog: "Delete image from layout?"
        - If confirmed, remove from images vector
        - Update z-order of remaining images
        - Clear image from cache
        - Deselect (no image selected afterward)
        - Show notification "Image removed"

4.  **Implement Paper Size and Margin Controls:**
    *   Paper size dropdown (`PickList` widget):
        - Options: All standard sizes (A0-A10, B0-B10, Letter, Legal, Tabloid, Ledger, Custom)
        - On selection change: `Message::PaperSizeChanged(size)`
        - In update: Update `layout.page` dimensions, adjust images that exceed new bounds
        - Show dimensions in parentheses (e.g., "A4 (210×297 mm)")
        - Default based on system locale (Letter for US, A4 for others)
    *   Paper type dropdown:
        - Options: MattePhoto, GlossPhoto, PhotoPaper, PrinterPaper, Satin, Canvas, RicePaper, Cardstock, Transparency
        - Store selection for printer configuration (Phase 4)
        - Display selected type in status bar
    *   Orientation toggle buttons (Portrait/Landscape):
        - Update page dimensions accordingly
        - Swap width/height values
        - Trigger canvas redraw
    *   Margin controls (4 number inputs):
        - Labels: "Top", "Bottom", "Left", "Right"
        - Spin boxes with up/down arrows and text input
        - Range: 0.0 mm to 50.0 mm
        - Step size: 0.5 mm with Shift for 1mm increments
        - "Link" toggle to set all margins equally
        - Preset buttons: "None (0mm)", "Small (5mm)", "Medium (10mm)", "Large (25mm)"
        - On change: `Message::MarginsChanged { top, bottom, left, right }`
        - Update page margins and redraw margin lines on canvas
        - Validate that margins don't exceed page dimensions
        - Show warning if margins exceed printable area
    *   **Custom Paper Size:**
        - When "Custom" selected, show width/height input fields
        - Units toggle: mm / inches
        - Save custom sizes to preferences for reuse

5.  **Implement Undo/Redo System:**
    *   Create `struct StateHistory` containing:
        - `states: Vec<Layout>` - Previous states
        - `current_index: usize`
        - `max_history: usize` - Limit to 50 states for memory efficiency
    *   On any layout modification, push current state to history
    *   Implement `undo()` and `redo()` methods
    *   Add Undo/Redo buttons to toolbar with keyboard shortcuts `Ctrl+Z` and `Ctrl+Shift+Z`
    *   Disable buttons when history is empty or at boundaries
    *   Show notification when undo/redo is triggered

6.  **Implement Zoom Controls:**
    *   Add zoom buttons to toolbar:
        - Zoom In (Ctrl+=)
        - Zoom Out (Ctrl+-)
        - Fit to Window (Ctrl+0)
        - Show current zoom percentage (e.g., "100%")
    *   On zoom change: `Message::ZoomChanged(factor)`
    *   Update canvas rendering with new zoom scale
    *   Maintain cursor position as zoom center (user-friendly)
    *   Store zoom preference in config for next session

7.  **Implement File Menu Operations:**
    *   Create menu items using Iced menu widgets (or use custom buttons styled as menu):
        - **File menu:**
            - New Layout - `Message::NewLayout`
            - Open Layout - `Message::OpenLayout` (Phase 5)
            - Save Layout - `Message::SaveLayout` (Phase 5)
            - Export as PDF - `Message::ExportPDF` (Phase 5)
            - Recent Files submenu (Phase 5)
            - Exit - `Message::Exit`
        - **Edit menu:**
            - Undo/Redo (already implemented)
            - Copy/Paste image (Phase 2 keyboard shortcuts)
        - **View menu:**
            - Show/Hide layers panel
            - Show/Hide properties panel
            - Grid overlay toggle
            - Show margins toggle
        - **Help menu:**
            - About
            - Keyboard Shortcuts dialog
            - Documentation link

8.  **Implement Status Bar:**
    *   Display at bottom of window showing:
        - Total images on layout: "Images: 3"
        - Current page: "Page: A4"
        - Zoom level: "100%"
        - Mouse coordinates when hovering canvas: "X: 50mm, Y: 75mm"
        - Last operation status: "Image moved" or "Unsaved changes"

9.  **Add Error and Success Notifications:**
    *   Implement notification system with toast messages:
        - Position: Top-right corner
        - Auto-dismiss after 3 seconds
        - Types: Success, Warning, Error
        - Examples:
            - "Image added successfully"
            - "Invalid image file format"
            - "Changes not saved"

10. **Implement Basic Image Adjustments (Optional for v0.1):**
    *   Add "Image Properties" panel when image selected:
        - Position X/Y (number inputs with mm/inch toggle)
        - Width/Height (with aspect ratio lock option)
        - Rotation angle (slider or number input)
        - Border width and color
    *   **Future consideration:** Basic adjustments like brightness/contrast
        - Defer complex image editing to external tools
        - Add "Edit in..." menu item to open in system image editor

11. **Test UI Implementation:**
    *   Verify all buttons, dropdowns, and inputs respond correctly
    *   Test keyboard shortcuts for all major functions
    *   Verify controls update canvas appropriately
    *   Test with various paper sizes and margin configurations
    *   Ensure UI is responsive with many images on layout
    *   Test undo/redo with multiple operations
    *   Test drag-and-drop from file manager
    *   Test multi-selection operations

---

### Phase 4: Printing Integration ✅ COMPLETE

**Goal:** Send the final layout to a physical printer using CUPS.

1.  **Research CUPS API and Dependencies:**
    *   **Primary approach:** Use `subprocess` with `lp` and `lpstat` commands (most portable, no library dependency)
    *   **Alternative:** `cups-rs` crate if available and maintained
    *   **Fallback strategy:** Document manual print workflow if CUPS unavailable
    *   Research CUPS daemon requirements and ensure it's running on target systems
    *   Create `src/printing.rs` module for all printer-related functionality
    *   Document printer capabilities needed: paper sizes, media types, resolutions, color modes
    *   **IPP Support:** Consider Internet Printing Protocol for network printers
    *   **Security Considerations:**
        - Sanitize all file paths passed to print commands
        - Validate printer names to prevent command injection
        - Use secure temporary file creation with proper permissions

2.  **Add Printer Discovery System in `src/printing.rs`:**
    *   Create `struct PrinterInfo` containing:
        - `name` - Printer identifier
        - `display_name` - Human-readable name
        - `is_default` - Whether this is the default printer
        - `location` - Physical location if available
        - `capabilities` - Supported features
        - `paper_sizes` - List of supported PaperSize values
        - `media_types` - List of supported PaperType values
        - `max_resolution` - Maximum DPI supported
    *   Implement `fn discover_printers() -> Vec<PrinterInfo>`:
        - Query CUPS daemon for available printers
        - Handle cases where CUPS is not running (show user message)
        - Filter out virtual/network printers if needed
        - Cache results for 30 seconds to avoid repeated queries
    *   Implement `fn get_default_printer() -> Option<PrinterInfo>`:
        - Determine system default printer
        - Fall back to first available printer if no default set
        - Store last used printer for future sessions

3.  **Implement Printer Selection UI:**
    *   Add printer selector to toolbar/properties panel:
        - `PickList` dropdown showing available printers
        - Button to refresh printer list with keyboard shortcut `Ctrl+R`
        - Show printer status (Online/Offline/Error)
    *   On printer selection: `Message::PrinterSelected(printer_name)`
    *   Update UI to show printer capabilities:
        - Filter paper size options to only those supported by printer
        - Filter paper type options to match printer specifications
        - Show maximum resolution capability
        - Disable paper sizes/types that aren't supported (with tooltip explanation)
    *   Handle printer becoming unavailable:
        - Show warning notification
        - Disable print button
        - Suggest selecting a different printer

4.  **Implement Print Preview:**
    *   Add "Preview" button to toolbar
    *   Create print preview dialog showing:
        - How layout will appear on selected paper
        - Actual colors and scaling
        - Margins and safe areas
        - Image resolution warning if images are too low DPI for quality print
        - Per-image DPI indicator overlay
    *   Implement `fn render_layout_to_image() -> ImageBuffer`:
        - Create temporary in-memory image at print resolution (DPI from printer)
        - Render page background (white)
        - Render each image at correct position and scale
        - Apply color space conversion if needed (sRGB for most printers)
        - Return the image buffer
    *   Show preview in modal dialog with ability to zoom and pan
    *   **Color Management (Basic):**
        - Default to sRGB color space for output
        - Future: ICC profile support for professional color accuracy
        - Note in documentation that color-critical work should use calibrated workflow

5.  **Implement Print Settings Dialog:**
    *   Create comprehensive print settings dialog with sections:
        - **Output**
            - Paper size (limited by printer capabilities)
            - Paper type (limited by printer capabilities)
            - Orientation (portrait/landscape)
            - Margins (validated against printer minimums)
        - **Quality**
            - Resolution dropdown (300 DPI, 600 DPI, 1200 DPI, etc.)
            - Color mode (Color, Grayscale, Monochrome)
            - Quality preset (Draft, Normal, Best)
        - **Layout**
            - Fit to page option (scale layout if needed)
            - Center on page option
        - **Advanced**
            - Media type details (for selected paper type)
            - Copies (1-999)
            - Collate option
            - Two-sided printing option (if supported)
    *   Validate all settings before allowing print:
        - Check resolution is supported by printer
        - Check paper size is available
        - Check margins don't exceed printer minimums
        - Warn if images have low resolution for selected DPI
    *   Store settings in config for next print job

6.  **Implement Layout Rendering Pipeline in `src/printing.rs`:**
    *   Create `struct PrintJob` containing:
        - `layout` - The Layout to print
        - `printer` - Target PrinterInfo
        - `settings` - PrintSettings
        - `temporary_file_path` - Path to rendered image
        - `job_id` - Optional CUPS job ID
    *   Implement `fn create_print_job() -> Result<PrintJob, Error>`:
        - Render layout to temporary image using `image` crate
        - Set image DPI metadata according to print settings
        - Save to temporary file (use `/tmp/print_layout_XXXXX.png`)
        - Validate file was created successfully
        - Return PrintJob or error with description
    *   Implement proper image composition:
        - Create ImageBuffer at correct dimensions for selected paper size at DPI
        - For each image in layout (in z-order):
            - Load image from disk
            - Scale to correct dimensions based on position/size in mm
            - Apply rotation if specified
            - Composite onto main buffer at correct position
            - Handle transparency/alpha properly
        - Validate final image has correct aspect ratio for paper
        - Compress if necessary to reasonable file size

7.  **Implement CUPS Integration for Actual Printing:**
    *   Create `fn send_to_printer(job: PrintJob) -> Result<JobId, Error>`:
        - Connect to CUPS daemon (handle connection failures gracefully)
        - Create print job with appropriate options:
            - Media size (e.g., "A4", "Letter")
            - Media type (e.g., "Photographic", "PlainPaper")
            - Resolution (e.g., "300x300dpi")
            - Number of copies
            - Two-sided printing option
        - Monitor job status (use polling or event listener)
        - Handle error conditions:
            - Printer offline - show user message
            - Unsupported media - fallback to compatible media with warning
            - Out of paper - suggest checking printer
            - Job submission failure - show error code and description
    *   Alternative: Fallback to `lp` command-line tool if CUPS library unavailable:
        ```bash
        lp -d printer_name -o media=A4 -o cpi=300 /tmp/file.png
        ```
    *   Implement job monitoring:
        - Poll CUPS for job status every 2 seconds
        - Show progress indicator: "Printing... (50%)"
        - Detect completion and show success message
        - Detect errors and show failure message

8.  **Implement Print Button and Workflow:**
    *   Add "Print" button to toolbar (large, prominent)
    *   On click: `Message::PrintClicked`
    *   In update() handle:
        1. Validate layout is not empty: `if layout.images.is_empty() { show warning }`
        2. Validate printer is selected and online
        3. Show print settings dialog (user can modify)
        4. On confirmation: Create print job in background task
        5. Show progress indicator while rendering and sending
        6. Handle success: Show notification, ask about saving layout
        7. Handle failure: Show error dialog with recovery options
    *   After successful print:
        - Clean up temporary file
        - Update job history
        - Optionally save layout automatically

9.  **Implement Print History and Queue Management:**
    *   Store print history in config file:
        - Printer used, date/time, paper size, number of copies
        - Number of images and total data size
        - Success/failure status
    *   Show print history in a panel or dialog:
        - List recent print jobs (last 20)
        - Re-print previous job option
        - Delete history entry option
    *   Monitor CUPS print queue:
        - Show pending/active jobs in status bar
        - Ability to cancel queued jobs from app

10. **Error Handling and Recovery:**
    *   Create comprehensive error types:
        ```rust
        enum PrintError {
            PrinterNotFound,
            PrinterOffline,
            UnsupportedMediaSize,
            InsufficientSpace,
            PermissionDenied,
            CUPSConnectionError,
            RenderingFailed,
            TemporaryFileError,
        }
        ```
    *   For each error, provide:
        - User-friendly message
        - Suggested resolution
        - Option to retry or select different printer

11. **Test Printing Integration:**
    *   Test with virtual printer (print to PDF if available)
    *   Test with real printer if possible
    *   Verify image rendering quality at various DPI settings
    *   Test all paper sizes and types
    *   Test error conditions:
        - Printer offline
        - Invalid paper configuration
        - File permission issues
        - CUPS daemon not running
    *   Verify temporary files are cleaned up properly (including on crash)
    *   Benchmark rendering time for layouts with many images
    *   **Security Testing:**
        - Test with malicious file paths
        - Verify temporary files have correct permissions (600)
        - Ensure cleanup on all exit paths

---

### Phase 5: Persistence & State Management 🔄 IN PROGRESS

**Goal:** Save and load user preferences and layout projects.

**Implemented:**
- Basic project save/load functionality
- JSON serialization for layouts
- File dialog integration

**Remaining:**
- Auto-save system
- Preferences dialog
- Recent files management
- Project backup system

1.  **Design Serialization Strategy in `src/config.rs`:**
    *   Create `struct UserPreferences` containing:
        - `last_printer: Option<String>` - Last used printer name
        - `default_paper_size: PaperSize` - User's preferred size
        - `default_paper_type: PaperType` - User's preferred type
        - `default_margins: (f32, f32, f32, f32)` - Top, bottom, left, right
        - `last_open_directory: PathBuf` - For file dialogs
        - `zoom_level: f32` - Last zoom percentage
        - `window_size: (u32, u32)` - Last window dimensions
        - `window_position: Option<(i32, i32)>` - Last window position
        - `ui_panels_visible: (bool, bool, bool)` - Visibility of panels
        - `recent_files: Vec<PathBuf>` - Last 10 opened layouts
        - `auto_save_enabled: bool` - Whether to auto-save periodically
        - `auto_save_interval_seconds: u32` - Default 5 minutes
        - `show_dpi_warnings: bool` - Whether to show low DPI warnings
        - `snap_to_grid: bool` - Enable/disable snapping
        - `grid_size_mm: f32` - Grid size for snapping
        - `measurement_units: Units` - mm or inches
        - `locale: Option<String>` - Override system locale
        - `custom_paper_sizes: Vec<CustomPaperSize>` - User-defined paper sizes
    *   Add serialization derives to all structures (already done in Phase 2)
    *   Create `struct ProjectLayout` for saving complete layouts:
        - `version: String` - Project file version (e.g., "0.1.0")
        - `layout: Layout` - The complete layout state
        - `created_at: String` - ISO 8601 timestamp
        - `last_modified: String` - ISO 8601 timestamp
        - `name: String` - Project name
        - `description: String` - Optional description

2.  **Implement Configuration File Management:**
    *   Define config file location: `~/.config/print_layout/config.json` (XDG compatible)
    *   Implement `fn load_config() -> UserPreferences`:
        - Check if config file exists, if not create with defaults
        - Parse JSON file with error handling
        - Validate all values (DPI ranges, valid paths, etc.)
        - Return default config if file corrupted or missing
        - Log warning if config had to be migrated from older format
    *   Implement `fn save_config(prefs: &UserPreferences) -> Result<(), Error>`:
        - Create config directory if it doesn't exist
        - Serialize preferences to JSON with nice formatting
        - Atomic write (write to temp file, then rename)
        - Handle permission denied gracefully
        - Return error if write fails
    *   Implement `fn ensure_config_directory() -> Result<PathBuf, Error>`:
        - Follow XDG Base Directory specification
        - Create ~/.config/print_layout/ if needed
        - Create ~/.cache/print_layout/ for temporary files
        - Return error if unable to create directories

3.  **Implement Save Layout Functionality:**
    *   Add "Save" and "Save As" buttons to File menu
    *   Keyboard shortcut: `Ctrl+S` (Save), `Ctrl+Shift+S` (Save As)
    *   On save: `Message::SaveLayout` or `Message::SaveLayoutAs`
    *   In update():
        - If first save, show file dialog to choose location (use rfd)
        - File extension: `.pxl` (Print Layout)
        - Validate layout is not empty
        - Create ProjectLayout struct with current layout state
        - Serialize to JSON
        - Save to file with atomic write
        - Update window title to show filename (not unsaved)
        - Show success notification: "Layout saved to: /path/to/file.pxl"
        - Update recent files list
    *   Implement dirty state tracking:
        - Track if layout has been modified since last save
        - Show asterisk (*) in title if unsaved
        - Prompt user before closing if unsaved changes exist
    *   Handle save errors:
        - Permission denied: Show error and suggest different location
        - Disk full: Show warning about space
        - Invalid filename: Show validation error

4.  **Implement Open Layout Functionality:**
    *   Add "Open" button to File menu
    *   Keyboard shortcut: `Ctrl+O` (Note: conflicts with Add Image, resolve by using context)
    *   On open: `Message::OpenLayout`
    *   In update():
        - Show file dialog (rfd) with .pxl filter
        - On file selected: `Message::LayoutFileSelected(path)`
        - Attempt to read and parse JSON file
        - Validate layout structure and version compatibility
        - Check that referenced image files still exist (optional auto-locate)
        - Load layout into application state
        - Clear undo/redo history
        - Update window title with filename
        - Show success notification
    *   Handle open errors:
        - File not found: Check recent files for alternate locations
        - Invalid format: Show parsing error
        - Version mismatch: Attempt migration or show compatibility warning
        - Missing images: Show dialog with missing file list, option to locate files
    *   Update "Recent Files" menu:
        - Show last 10 opened files
        - Click to quickly re-open
        - "Clear Recent Files" option

5.  **Implement Auto-Save System:**
    *   Create background task that runs every N seconds (configurable, default 5 minutes)
    *   Only auto-save if:
        - Layout has been modified since last save
        - Auto-save is enabled in preferences
        - Not currently printing
    *   Save to special auto-save location: `~/.cache/print_layout/auto_save.pxl`
    *   On application crash/restart:
        - Detect auto-save file exists
        - Show dialog: "Recover unsaved layout from [timestamp]?"
        - Option to recover or discard
    *   Implement periodic cleanup:
        - Keep only 3 most recent auto-saves
        - Delete auto-saves older than 7 days

6.  **Implement Preferences Dialog:**
    *   Create UI dialog accessible from Edit menu or settings button
    *   Sections:
        - **General**
            - Default paper size (dropdown)
            - Default paper type (dropdown)
            - Default printer (dropdown, refreshes from CUPS)
            - Auto-save enabled (checkbox)
            - Auto-save interval (spin box, 1-30 minutes)
        - **Appearance**
            - Show grid overlay (checkbox)
            - Grid size (dropdown: 10mm, 5mm, 1mm)
            - Show margins (checkbox)
            - Canvas background (color picker)
            - Page background (color picker)
        - **Advanced**
            - Temporary files location (file picker)
            - Cache size limit (slider)
            - Debug logging (checkbox)
            - Clear cache button
    *   Validate inputs and apply immediately
    *   Show "Apply" and "Cancel" buttons
    *   Save preferences on apply

7.  **Implement Recent Files Management:**
    *   Maintain recent files list in config
    *   Add to list every time a layout is opened or saved
    *   Move opened file to top of list
    *   Limit to 10 most recent
    *   Verify files still exist before showing in menu
    *   "Recent Files" submenu in File menu
    *   "Clear Recent Files" option to reset

8.  **Implement Project Backup System:**
    *   On every save, create backup of previous version
    *   Store backups in `~/.config/print_layout/backups/`
    *   Keep last 5 backups per file
    *   Naming: `filename_backup_2025-11-28_143022.pxl`
    *   Option to restore from backup via File menu
    *   Auto-cleanup old backups (keep only 5)

9.  **Handle Image Path References:**
    *   When saving layout, use relative paths when possible:
        - If image in same directory as layout file: Use relative path
        - Otherwise: Store absolute path or let user choose
    *   When loading layout:
        - Try relative path first
        - Fall back to absolute path
        - If not found, show dialog to locate image
        - Offer to search in common directories
        - Update layout with new path if found
    *   Validation:
        - Warn if images are on different drives (may not be portable)
        - Suggest copying images to layout directory for portability

10. **Implement State Update Listeners:**
    *   Add event system to track when layout is modified
    *   Methods that trigger modification:
        - Add/remove image
        - Move/resize image
        - Change page settings
        - Rotate image
        - Change z-order
    *   Each modification:
        - Marks layout as "dirty"
        - Triggers auto-save timer reset
        - Updates window title if needed

11. **Test Persistence Implementation:**
    *   Test save/load cycle with various layouts
    *   Verify recent files list works
    *   Test auto-save by waiting for interval
    *   Test recovery from auto-save after restart
    *   Verify config migration if format changes
    *   Test backup creation and restoration
    *   Test relative and absolute image paths
    *   Test with missing image files on load
    *   Verify no corruption on crashes
    *   Performance test: Load/save large layouts (50+ images)
    *   **Security Testing:**
        - Test with paths containing special characters
        - Verify config file permissions (600 or 644)
        - Test behavior with read-only config directory

---

### Phase 6: Packaging & Final Touches ⬜ NOT STARTED

**Goal:** Polish the application and package it for distribution.

1.  **Refine User Interface:**
    *   **Visual Polish**
        - Implement consistent color scheme throughout app
        - Create application icon (256x256 SVG and PNG, multiple sizes for different contexts)
        - Add visual feedback for all interactive elements:
            - Hover states for buttons
            - Active/inactive states
            - Loading spinners for long operations
            - Progress bars for file operations
        - Improve spacing and alignment across all panels
        - Use consistent typography with readable font sizes
        - Support system dark/light theme detection
    *   **Accessibility**
        - Ensure all UI elements have proper labels and alt text
        - Support keyboard navigation (Tab through controls)
        - Implement proper focus indicators
        - Use sufficient color contrast (WCAG AA compliance)
        - Test with screen readers if possible
        - Support high-DPI displays
        - Ensure all functionality accessible via keyboard
    *   **Internationalization (i18n) Preparation:**
        - Use string constants for all user-visible text
        - Structure code to support future translation
        - Document locale-specific behaviors (paper sizes, units)
        - Note: Full translation support deferred to future version

2.  **Implement Comprehensive Error Handling:**
    *   Create error dialog system showing:
        - Clear, non-technical error messages
        - Suggested actions for recovery
        - Technical details in expandable section for debugging
    *   Specific handlers for common errors:
        - **File errors:**
            - File not found: "Image file could not be found at [path]"
              Action: "Browse for image" button
            - Permission denied: "No permission to access [file]"
              Action: "Check file permissions" button
            - File too large: "Image exceeds maximum size (100MB)"
              Action: "Reduce image size or try different file"
        - **Printer errors:**
            - Printer offline: "Selected printer is offline"
              Action: "Try different printer" or "Retry connection"
            - No printers available: "No printers detected"
              Action: "Install printer driver" or "Start CUPS daemon"
            - Paper not available: "Requested paper size not supported"
              Action: "Select compatible paper"
        - **System errors:**
            - Out of memory: "Insufficient memory for operation"
              Action: "Close other applications" or "Reduce layout size"
            - Disk full: "Insufficient disk space for print job"
              Action: "Free disk space" or "Retry later"
    *   Log all errors to file: `~/.cache/print_layout/app.log`
    *   Include "Report Issue" button that opens issue URL with pre-filled details

3.  **Implement User Help System:**
    *   Create context-sensitive help:
        - Tooltips on all buttons (show on hover after 500ms)
        - "?" buttons on complex UI sections
        - Help panel with searchable topics
    *   Create in-app tutorial:
        - On first launch, show welcome dialog with quick start guide
        - Optional guided tour of major features
        - Can be re-opened from Help menu
    *   Create user manual:
        - Markdown file converted to HTML for in-app viewing
        - Topics: Getting started, basic layout, advanced features, troubleshooting
        - Keyboard shortcuts reference
    *   Add keyboard shortcuts dialog:
        - Accessible from Help menu and by pressing `?`
        - Organized by category (File, Edit, View, Printing)
        - Searchable/filterable
    *   Online documentation link pointing to web resource

4.  **Performance Optimization:**
    *   **Canvas Rendering**
        - Implement dirty rectangle tracking (only redraw changed areas)
        - Cache rendered pages when nothing changes
        - Use GPU acceleration if available (via Iced)
        - Lazy-load images only when visible on canvas
    *   **Memory Management**
        - Implement image caching with size limits
        - Unload cached images when not used for 5 minutes
        - Monitor memory usage and warn if exceeding threshold
        - Profile memory with large layouts (100+ images)
    *   **File Operations**
        - Load/save layouts in background threads (already async)
        - Show progress for large file operations
        - Implement cancellation for long-running tasks
    *   **Startup Time**
        - Measure and optimize startup time (target <1 second)
        - Lazy-load CUPS printer list (show loading state)
        - Cache printer list for 30 seconds
    *   **Benchmarking**
        - Document rendering performance metrics
        - Add performance test suite

5.  **Implement Logging and Debugging:**
    *   Create logging system using `log` and `env_logger` crates
    *   Log levels: Error, Warn, Info, Debug, Trace
    *   Log file location: `~/.cache/print_layout/app.log`
    *   Implement log rotation (keep last 5 files, max 10MB each)
    *   Command-line argument for debug mode: `--debug` or `--trace`
    *   Debug features:
        - Show render statistics overlay (FPS, memory, image count)
        - Show canvas coordinate grid
        - Visualize hit detection areas
        - Export debug information command

6.  **Create Comprehensive Testing Suite:**
    *   **Unit Tests**
        - Test layout calculations and transformations
        - Test paper size/type conversions
        - Test serialization/deserialization
        - Test image cache functionality
        - Test margin calculations
        - Target coverage: 70%+
    *   **Integration Tests**
        - Test save/load cycles with various layouts
        - Test printer discovery and selection
        - Test print job creation
        - Test config file management
    *   **Manual Testing Checklist**
        - [ ] All UI elements respond correctly
        - [ ] Keyboard shortcuts all work
        - [ ] Can add/move/resize/delete images
        - [ ] Can change page size and margins
        - [ ] Can save and load layouts
        - [ ] Print preview works
        - [ ] Printing to virtual printer succeeds
        - [ ] Error dialogs display properly
        - [ ] Undo/redo works for all operations
        - [ ] Application handles missing images gracefully
        - [ ] Application handles offline printer gracefully
        - [ ] Memory usage reasonable with 50+ images
        - [ ] Application starts in <1 second
    *   **Platform Testing**
        - Test on Fedora/RHEL
        - Test on Ubuntu/Debian
        - Test on Arch Linux
        - Test on both X11 and Wayland

7.  **Create Installation and Build Documentation:**
    *   Update `Cargo.toml` with:
        - Complete metadata (author, description, license, etc.)
        - Categories and keywords
        - Repository link
        - Documentation link
    *   Create `README.md` with:
        - Brief project description
        - Feature list
        - Screenshots
        - Installation instructions for each distro
        - Usage quick start
        - Keyboard shortcuts reference
        - Known limitations
        - Contributing guidelines
        - License information
    *   Create `INSTALL.md` with:
        - System requirements
        - Build from source instructions
        - Dependency installation for common distros
        - Troubleshooting common build issues
    *   Create `USAGE.md` with:
        - Feature documentation
        - Step-by-step tutorials
        - FAQ section
        - Keyboard shortcuts
        - Tips and tricks
    *   Create `DEVELOPMENT.md` with:
        - Code structure overview
        - Module documentation
        - How to add new paper sizes
        - How to extend printer support
        - How to add new image formats

8.  **Prepare for Distribution:**
    *   **Build Optimization**
        - Set up `.cargo/config.toml` for release builds:
            ```toml
            [profile.release]
            opt-level = 3
            lto = true
            codegen-units = 1
            strip = true
            panic = "abort"
            ```
        - Test release build: `cargo build --release`
        - Binary size target: <15MB
    *   **Cross-Compilation:**
        - Set up cross-compilation for ARM64 (aarch64-unknown-linux-gnu)
        - Test on Raspberry Pi or ARM-based systems
        - Document any platform-specific limitations
    *   **AppImage Creation**
        - Install `cargo-appimage` or `appimagetool`
        - Create AppImage that runs on any Linux distro
        - Include all dependencies in AppImage
        - Create .desktop file with proper categories
        - Test on different distros (Ubuntu, Fedora, Arch)
    *   **Debian Package**
        - Create `debian/` directory with control files
        - Build .deb with `cargo-deb`
        - Test installation with `dpkg`
        - Specify dependencies: libcups2, etc.
    *   **Flatpak (Future):**
        - Consider Flatpak for sandboxed distribution
        - Document portal requirements for printing
    *   **Checksums and Signing**
        - Generate SHA256 checksums for releases
        - Create GPG signatures (optional for initial release)
        - Document verification process

9.  **Create Release Package Structure:**
    ```
    print-layout-0.1.0/
    ├── README.md
    ├── INSTALL.md
    ├── USAGE.md
    ├── CHANGELOG.md
    ├── LICENSE
    ├── print-layout-0.1.0-x86_64.AppImage
    ├── print-layout_0.1.0_amd64.deb (if applicable)
    ├── print-layout-0.1.0.tar.gz (source)
    └── SHA256SUMS
    ```

10. **Prepare Release Documentation:**
    *   Create `CHANGELOG.md` documenting:
        - Version 0.1.0 initial release
        - All features implemented
        - Known limitations
        - Future planned features
    *   Create release notes highlighting:
        - Major features
        - System requirements
        - Installation instructions
        - Quick start guide
    *   Document known limitations:
        - Maximum images per layout (if applicable)
        - Supported image formats
        - Supported paper sizes
        - Supported printers

11. **Final Quality Assurance:**
    *   **Automated Quality Checks**
        - Run `cargo clippy` for linting
        - Run `cargo fmt --check` for code formatting
        - Run `cargo test` to verify all tests pass
        - Build documentation: `cargo doc`
    *   **Manual Testing**
        - Verify all features work as documented
        - Test on target platforms
        - Verify help system is complete
        - Verify error messages are helpful
        - Test with various layouts (empty, 1 image, 50+ images)
    *   **User Experience Review**
        - Is workflow intuitive?
        - Are error messages clear?
        - Are all features discoverable?
        - Is performance acceptable?
        - Is documentation clear?

12. **Prepare for Launch:**
    *   Create project website or landing page
    *   Set up issue tracker/bug reports
    *   Create community forum or discussion space
    *   Write announcement for Linux communities
    *   Tag release in version control
    *   Upload binaries and packages to hosting
    *   Document installation on landing page
    *   Set up automated builds/CI for future versions
    *   Plan future features and roadmap
    *   Document maintenance strategy

13. **Post-Launch Monitoring:**
    *   Monitor bug reports and user feedback
    *   Create priority list for bug fixes
    *   Plan next version improvements
    *   Maintain changelog
    *   Provide regular updates and patches
    *   Document user tips and tricks
    *   Consider adding user survey for feature requests

---

### Future Considerations (Post v1.0)

The following features are out of scope for the initial release but should be considered for future versions:

1.  **Multi-Page Support:**
    - Multiple pages in a single project
    - Page navigation and thumbnails
    - Booklet printing mode

2.  **Advanced Color Management:**
    - ICC profile support for input images
    - Printer ICC profile integration
    - Soft proofing mode
    - CMYK preview

3.  **Template System:**
    - Pre-built layout templates (photo collages, business cards, etc.)
    - User-saved templates
    - Template marketplace/sharing

4.  **Advanced Image Editing:**
    - In-app cropping tool
    - Basic adjustments (brightness, contrast, saturation)
    - Simple filters and effects
    - Red-eye removal

5.  **RAW Image Support:**
    - Camera RAW file formats (CR2, NEF, ARW, etc.)
    - Requires integration with libraw or similar
    - Significant development effort

6.  **PDF Export:**
    - Export layout as PDF for sharing
    - PDF/X compliance for professional printing
    - Embedded fonts and color profiles

7.  **Cloud Integration:**
    - Cloud printing services
    - Cloud storage for layouts
    - Sync preferences across devices

8.  **Full Localization:**
    - Complete UI translation
    - RTL language support
    - Localized documentation

9.  **Plugin System:**
    - Extensible architecture for community plugins
    - Custom paper sizes via plugins
    - Additional export formats

10. **Batch Processing:**
    - Print multiple layouts in sequence
    - Batch import and layout generation
    - Contact sheet generation
