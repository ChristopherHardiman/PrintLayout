# PrintLayout Upgrade Plan: Tauri + Web Stack Migration

**Author:** PrintLayout Team  
**Date:** 2025-12-04  
**Current Version:** 0.2.1  
**Target Version:** 0.3.0  
**Branch:** gui_rework  
**Objective:** Replace Iced-based UI with Tauri + Web stack while preserving all existing functionality

---

## Executive Summary

This document provides a detailed, actionable upgrade plan for migrating PrintLayout from its current Iced-based GUI to a Tauri + Web technology stack. The migration aims to deliver a more polished, Canon Professional Print & Layout-inspired interface while maintaining all existing functionality and improving canvas performance.

### Core Goals
1. **Preserve Business Logic** - All Rust printing/CUPS/layout code remains intact
2. **Improve Visual Design** - Match Canon PPL's professional aesthetic with CSS flexibility
3. **Enhance Canvas Performance** - Leverage GPU-accelerated web rendering for smooth interactions
4. **Maintain Feature Parity** - Zero functionality loss during transition
5. **Enable Rapid UI Evolution** - Web stack allows easier styling iterations

### Migration Strategy
- **Incremental Approach:** Build Tauri shell alongside existing Iced code
- **Hard Switch:** Remove Iced entirely (separate branch reduces rollback risk)
- **Parallel Testing:** Validate Tauri implementation against v0.2.1 baseline
- **Staged Rollout:** Beta release before full transition

### Design Targets
- **Canon PPL Aesthetic:** Target 80% visual similarity (professional look without pixel-perfect matching)
- **Performance First:** Validate canvas rendering strategy in Phase 0 before full implementation

---

## 0. Phase 0: Canvas Rendering Prototype (Pre-Work)

**Objective:** Determine optimal canvas rendering strategy before committing to full architecture.

**Duration:** 3-5 days

### 0.1 Prototype Goals
1. Measure canvas rendering latency for both approaches
2. Evaluate memory usage with realistic layouts (10+ images)
3. Test IPC overhead for bitmap streaming
4. Validate image loading/caching strategies

### 0.2 Prototype A: Pure Web Rendering
**Implementation:**
- Minimal Tauri shell with single command: `get_layout_data() -> LayoutDTO`
- Load layout with 5-10 test images
- Decode images in browser as `Image` objects
- Render page + images on Canvas2D
- Apply transforms (rotation, flip, opacity) client-side

**Metrics to Collect:**
- Time to first render (cold start)
- Frame time during pan/zoom (target: <16ms)
- Memory usage (baseline + per image)
- Image decode time (by file size)

### 0.3 Prototype B: Rust-Rendered Bitmaps
**Implementation:**
- Tauri command: `render_canvas(layout: LayoutDTO, zoom: f32) -> Vec<u8>`
- Reuse existing `canvas_widget.rs` rendering logic
- Stream RGBA bitmap to frontend via IPC
- Display on canvas with `drawImage()`

**Metrics to Collect:**
- Render time in Rust (server-side)
- IPC transfer time (by resolution)
- Frame time during interactions
- Memory usage (frontend + backend)

### 0.4 Decision Criteria
| Metric | Pure Web Target | Rust Bitmap Target | Weight |
|--------|----------------|-------------------|--------|
| Frame time (pan/zoom) | <16ms | <16ms | High |
| Cold start | <1s | <2s | Medium |
| Memory (10 images) | <500MB | <400MB | Medium |
| IPC overhead | N/A | <50ms | High |
| Implementation complexity | Medium | Low | Low |

**Decision Rule:**
- If Pure Web meets all targets → Use Pure Web (better long-term flexibility)
- If Rust Bitmap significantly faster → Use Rust Bitmap (leverage existing code)
- If both fail → Hybrid approach (Rust renders, Web handles UI overlays)

### 0.5 Prototype Deliverables
- Performance comparison document
- Recommended approach with justification
- Identified optimizations for chosen strategy
- Updated Section 4.3 with implementation details

**Exit Criteria:** Team approval of rendering strategy before Week 1 begins.

---

## 1. Current Architecture Assessment

### 1.1 Application Structure
**Current State (v0.2.1):**
- **UI Framework:** Iced 0.13 with MVU (Model-View-Update) architecture
- **Module Organization:**
  - `src/main.rs` (1928 lines) - Application state, message handling, view rendering
  - `src/canvas_widget.rs` - Custom canvas with image rendering and transform cache
  - `src/layout.rs` - Core data structures (Page, PlacedImage, Layout)
  - `src/printing.rs` - CUPS integration and printer capabilities
  - `src/config.rs` - Configuration persistence and project file management
  - `src/lib.rs` - Library exports

### 1.2 State Management
**PrintLayout Struct (52 fields):**
- Layout state: `layout`, `canvas`, `zoom`
- Printer state: `printers`, `selected_printer`, `printer_capabilities`, CUPS option selections
- UI state: `settings_tab`, `print_status`, drag modes, input field values
- File state: `current_file`, `project`, `is_modified`, auto-save tracking
- Caches: `thumbnail_cache`, transform caches in canvas widget

### 1.3 Message Handling
**Current Message Enum (47 variants):**
- Canvas interactions: `SelectImage`, `StartResize`, `MouseMoved`, `MouseReleased`, etc.
- File operations: `NewLayout`, `SaveLayoutClicked`, `OpenLayoutClicked`, `OpenRecentFile`
- Image manipulation: `RotateImageCW/CCW`, `FlipImage*`, `ImageOpacityChanged`, resize operations
- Printing: `PrintersDiscovered`, `PrinterSelected`, `PrinterCapabilitiesLoaded`, `PrintClicked`
- Settings: `PaperSizeSelected`, `MarginChanged`, `SettingsTabChanged`, CUPS option selections
- UI controls: Zoom, thumbnails, dialogs

### 1.4 Key Features to Preserve
1. **Layout Engine**
   - 50+ paper size presets (A-series, B-series, photo sizes, North American)
   - Custom margins with validation
   - Portrait/landscape orientation switching
   - Borderless printing support

2. **Image Management**
   - Multi-file selection with async file dialogs
   - Drag-and-drop positioning on canvas
   - 8-handle resize (corners + edges) with aspect ratio lock
   - Transform operations: rotate (90° increments), flip horizontal/vertical
   - Opacity control (0-100%)
   - Image caching with transform-based keys

3. **CUPS Integration**
   - Dynamic printer discovery via `lpstat -p -d`
   - Printer capabilities querying via `lpoptions -p <printer> -l`
   - Dynamic dropdowns for: InputSlot, MediaType, ColorModel, PrintQuality
   - High-resolution rendering (300 DPI default)
   - Async print job execution with status tracking

4. **Persistence**
   - Project file format (.pxl JSON with versioning)
   - Auto-save every 30 seconds with recovery dialog
   - Configuration persistence (XDG-compliant)
   - Recent files tracking (up to 10)
   - Automatic backups (5 most recent)

5. **UI Layout (Canon PPL-inspired)**
   - Top bar: printer selection, file operations
   - Toolbar: add/delete images, zoom controls, orientation toggle
   - Left panel: image thumbnails with horizontal scrolling
   - Center: canvas with rulers, guides, selection highlights
   - Right panel: tabbed settings (Print Settings, Layout, Image Tools)
   - Bottom: status bar with zoom percentage

---

## 2. Tauri Architecture Design

### 2.0 Key Architectural Decisions

**Image File Access Strategy:**
- Use Tauri's asset protocol to serve local image files
- Copy user-selected images to Tauri's asset directory on `add_images()`
- Serve via `asset://` protocol to WebView (bypasses CORS/security restrictions)
- Maintain original file paths in layout data for printing
- Clean up asset directory on app close or via periodic garbage collection

**Dependency Migration:**
- Remove Iced framework dependency entirely (hard switch on `gui_rework` branch)
- Audit remaining Iced-dependent code:
  - `canvas_widget.rs` → Adapt for backend rendering or rewrite for web
  - Image caching → Consolidate into single cache (see 2.0.1)
  - Any Iced types in DTOs → Replace with framework-agnostic types
- Replace `rfd::AsyncFileDialog` with Tauri's `dialog` API

**Cache Consolidation Plan (2.0.1):**
Current: 3 separate caches
- `thumbnail_cache` (in main.rs)
- `ImageCache` (transformed images in canvas_widget.rs)
- `SourceImageCache` (original images in canvas_widget.rs)

Proposed: 2 unified caches
- **Backend `ImageStore`**: Original + transformed images with LRU eviction
- **Frontend Cache**: Browser-native image cache via `asset://` URLs

Benefits: Reduced memory, simpler lifecycle management, fewer cache invalidation bugs

**CUPS Integration:**
- Keep subprocess-based CUPS calls (`lpstat`, `lp`, `lpoptions`) in backend
- Test in Tauri context during Phase 0 or Week 1
- No changes needed to CUPS logic (already framework-agnostic)
- Add integration test: `test_cups_in_tauri_context()`

**Dialog APIs:**
- Migrate from `rfd` to `tauri::api::dialog`
- Commands: `dialog::FileDialogBuilder` for open/save
- Benefits: Better integration, consistent with Tauri conventions

**Keyboard Shortcuts:**
- Register global shortcuts in `tauri.conf.json`
- Define shortcuts: Ctrl+Z (undo), Ctrl+S (save), Ctrl+O (open), etc.
- Emit events to frontend on shortcut trigger
- Frontend calls appropriate command in response

**HiDPI Support:**
- Implement `devicePixelRatio` scaling in canvas initialization
- Set canvas internal resolution: `width = clientWidth * devicePixelRatio`
- Test on 4K display during Week 6 (polish phase)
- Add to acceptance criteria

**Data Migration & Cleanup:**
- Add initialization check: `check_legacy_config()` on app start
- Detect old Iced-specific config files
- Prompt user to migrate or clean up (one-time dialog)
- Use different config subdirectory: `~/.local/share/print_layout/tauri/` to avoid conflicts

**Backward Compatibility:**
- Add `.pxl` file format compatibility test
- Ensure v0.2.1 project files load correctly in v0.3.0
- Add integration test: `test_load_v021_project()`
- Version migration handled in `config` crate

### 2.1 Project Structure
```
PrintLayout/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Tauri app initialization, command registration
│   │   ├── commands/       # Tauri command modules (new)
│   │   │   ├── mod.rs
│   │   │   ├── printer.rs  # Printer discovery, capabilities
│   │   │   ├── layout.rs   # Layout operations
│   │   │   ├── image.rs    # Image loading, manipulation
│   │   │   ├── file.rs     # Save/load project files
│   │   │   └── render.rs   # Canvas rendering helpers
│   │   ├── state/          # Application state management (new)
│   │   │   ├── mod.rs
│   │   │   └── app_state.rs
│   │   └── dto/            # Data Transfer Objects (new)
│   │       ├── mod.rs
│   │       ├── layout_dto.rs
│   │       ├── printer_dto.rs
│   │       └── image_dto.rs
│   ├── crates/             # Existing modules (refactored)
│   │   ├── layout/         # layout.rs becomes standalone crate
│   │   ├── printing/       # printing.rs becomes standalone crate
│   │   ├── config/         # config.rs becomes standalone crate
│   │   └── canvas/         # canvas_widget.rs adapted for backend rendering
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # Web frontend (HTML/CSS/JS)
│   ├── index.html
│   ├── styles/
│   │   ├── main.css
│   │   ├── canon-theme.css
│   │   └── components/
│   ├── js/
│   │   ├── main.js
│   │   ├── canvas.js       # Canvas rendering logic
│   │   ├── state.js        # Frontend state management
│   │   └── api.js          # Tauri command wrappers
│   └── assets/
│       └── icons/
├── package.json            # Node tooling config
└── Cargo.toml              # Workspace config
```

### 2.2 Command Surface Design
**Required Tauri Commands (grouped by functionality):**

**Printer Commands:**
- `discover_printers() -> Vec<PrinterInfoDTO>`
- `get_printer_capabilities(printer: String) -> PrinterCapabilitiesDTO`
- `execute_print_job(job: PrintJobDTO) -> Result<String, String>`

**Layout Commands:**
- `new_layout() -> LayoutDTO`
- `update_paper_size(size: PaperSizeDTO) -> ()`
- `update_margins(top, bottom, left, right: f32) -> ()`
- `toggle_orientation() -> LayoutDTO`
- `toggle_borderless(enabled: bool) -> ()`

**Image Commands:**
- `add_images(paths: Vec<String>) -> Vec<PlacedImageDTO>`
- `remove_image(id: String) -> ()`
- `update_image_position(id: String, x: f32, y: f32) -> ()`
- `update_image_size(id: String, width: f32, height: f32) -> ()`
- `rotate_image(id: String, degrees: f32) -> ()`
- `flip_image(id: String, horizontal: bool, vertical: bool) -> ()`
- `update_image_opacity(id: String, opacity: f32) -> ()`
- `select_image(id: String) -> ()`

**File Commands:**
- `save_project(path: String) -> Result<(), String>`
- `load_project(path: String) -> Result<ProjectLayoutDTO, String>`
- `get_recent_files() -> Vec<String>`
- `auto_save() -> Result<(), String>`
- `check_recovery_file() -> Option<String>`

**Rendering Commands:**
- `render_canvas_preview(layout: LayoutDTO, zoom: f32) -> Vec<u8>` (RGBA bitmap)
- `get_image_thumbnail(path: String) -> Vec<u8>`

**Configuration Commands:**
- `load_preferences() -> UserPreferencesDTO`
- `save_preferences(prefs: UserPreferencesDTO) -> ()`

**Event Emissions (Rust → Web):**
- `print-progress` (status: String, percentage: f32)
- `auto-save-complete` (success: bool)
- `error-occurred` (message: String)

### 2.3 Data Transfer Objects (DTOs)
All DTOs must be `Serialize + Deserialize` for JSON over IPC.

**Key DTOs to Define:**
- `LayoutDTO` - mirrors `Layout` struct
- `PlacedImageDTO` - mirrors `PlacedImage`
- `PageDTO` - mirrors `Page`
- `PrinterInfoDTO` - mirrors `PrinterInfo`
- `PrinterCapabilitiesDTO` - mirrors `PrinterCapabilities`
- `PrintJobDTO` - command parameters for printing
- `ProjectLayoutDTO` - mirrors `ProjectLayout`
- `UserPreferencesDTO` - mirrors `UserPreferences`

---

## 3. Backend Refactoring Tasks

### 3.1 Extract Core Modules into Crates
**Objective:** Decouple business logic from UI framework dependencies.

**Tasks:**
1. **Create `crates/layout` workspace member**
   - Move `layout.rs` into `crates/layout/src/lib.rs`
   - Remove any Iced-specific imports/dependencies
   - Export all public types (PaperSize, Page, PlacedImage, Layout, etc.)
   - Add unit tests for layout calculations

2. **Create `crates/printing` workspace member**
   - Move `printing.rs` into `crates/printing/src/lib.rs`
   - Ensure CUPS integration is framework-agnostic
   - Export printer discovery, capabilities, and print execution functions
   - Add integration tests for CUPS commands

3. **Create `crates/config` workspace member**
   - Move `config.rs` into `crates/config/src/lib.rs`
   - Remove UI-specific state (zoom, drag modes, input fields)
   - Keep persistence logic (ProjectLayout, UserPreferences, ConfigManager)
   - Add tests for serialization/deserialization

4. **Create `crates/canvas` workspace member**
   - Adapt `canvas_widget.rs` for backend rendering
   - Provide functions to generate preview bitmaps
   - Keep image caching logic (transform-based keys)
   - Expose rendering API for Tauri commands

### 3.2 Define DTO Layer
**Objective:** Create JSON-serializable versions of all data structures.

**Tasks:**
1. **Create `src-tauri/src/dto/` module**
   - Define DTOs matching core types (1:1 mapping)
   - Implement `From<Layout> for LayoutDTO` and reverse conversions
   - Use `#[derive(Serialize, Deserialize)]` on all DTOs
   - Document any field transformations (e.g., PathBuf → String)

2. **Optional: Use `ts-rs` crate**
   - Add `#[derive(TS)]` to DTOs for TypeScript bindings generation
   - Generate `.d.ts` files for frontend type safety
   - Integrate generation into build pipeline

### 3.3 Implement Tauri Commands
**Objective:** Expose backend functionality via async commands callable from web UI.

**Tasks:**
1. **Create `src-tauri/src/commands/` module structure**
   - Organize commands by domain (printer, layout, image, file, render)
   - Each command function annotated with `#[tauri::command]`
   - All commands return `Result<T, String>` for error handling

2. **Implement printer commands**
   - Wrap existing `discover_printers()` and `get_printer_capabilities()`
   - Convert results to DTOs before returning
   - Add error messages for CUPS failures

3. **Implement layout/image commands**
   - Maintain central `AppState` (Mutex-wrapped layout)
   - Commands modify state and return updated DTO
   - Ensure thread-safe access to shared state

4. **Implement file commands**
   - Reuse existing `ConfigManager` logic
   - Add async file I/O for save/load operations
   - Handle path conversion (String ↔ PathBuf)

5. **Implement rendering commands**
   - Generate canvas preview as RGBA bitmap
   - Apply zoom/transform for preview generation
   - Return base64-encoded image or raw bytes

### 3.4 State Management Strategy
**Objective:** Centralize application state for command access.

**Options:**
- **Option A: Tauri Managed State**
  - Use `tauri::State<Mutex<AppState>>` in commands
  - Single source of truth in Rust
  - Web UI sends all changes via commands
  - **Recommended for simplicity**

- **Option B: Hybrid State**
  - Rust manages core data (layout, images)
  - Web UI manages ephemeral UI state (drag positions, input values)
  - Requires careful synchronization

**Recommendation:** Start with Option A for consistency.

---

## 4. Frontend Implementation Plan

### 4.1 Initial Setup
**Tasks:**
1. **Initialize Tauri project**
   - Run `cargo tauri init --frontend vanilla` in repo root
   - Configure `tauri.conf.json`:
     - Set app name, version, identifier
     - Configure window size (1400×900 default)
     - Enable required permissions (filesystem for images, dialogs)
   - Update workspace `Cargo.toml` to include `src-tauri`

2. **Set up Node tooling**
   - Create `package.json` with minimal dependencies
   - Add scripts: `dev`, `build`, `lint`
   - Optional: Add Vite for dev server and hot reload

3. **Create base HTML structure**
   - `src/index.html` with semantic layout:
     - `<header>` - top bar (printer, file ops)
     - `<nav>` - toolbar (add/delete, zoom, orientation)
     - `<aside class="thumbnails">` - left panel
     - `<main>` - canvas area
     - `<aside class="settings">` - right panel with tabs
     - `<footer>` - status bar

### 4.2 CSS Styling (Canon PPL Theme)
**Objectives:**
- Professional, polished aesthetic matching Canon's design language
- Subtle gradients, layered panels, clean typography
- Responsive layout for 1080p and 4K displays

**Tasks:**
1. **Define CSS variables (canon-theme.css)**
   - Color palette: primary, secondary, accent, text colors
   - Spacing: consistent padding/margins (4px, 8px, 12px, 16px, 24px)
   - Typography: font families, sizes, weights
   - Shadows: subtle elevation for panels

2. **Style base layout (main.css)**
   - Flexbox for panel arrangement
   - Fixed header/footer, flexible main area
   - Scrollable thumbnail and settings panels
   - Canvas area fills remaining space

3. **Create component styles**
   - Buttons: primary, secondary, icon-only variants
   - Dropdowns: custom-styled selects or native with consistent borders
   - Text inputs: unified styling with focus states
   - Tabs: segmented control appearance
   - Thumbnails: bordered cards with hover effects
   - Modals/dialogs: centered overlays with backdrop blur

4. **Canvas styling**
   - `<canvas>` element sized to fit parent
   - Ruler overlays as positioned divs or SVG
   - Selection outlines: CSS borders or SVG rect
   - Resize handles: positioned absolutely, styled as small squares

### 4.3 JavaScript/Canvas Rendering
**Objectives:**
- Smooth pan/zoom interactions
- Drag-and-drop image positioning
- Resize handle manipulation
- Decision: WebGL vs Canvas2D

**Tasks:**
1. **Canvas initialization (canvas.js)**
   - Get 2D context (or WebGL context if performance demands)
   - Set canvas dimensions matching physical pixels (devicePixelRatio)
   - Initialize render loop (requestAnimationFrame)

2. **Rendering strategy decision**
   - **Option A: Pure Web Rendering**
     - Fetch layout data from Rust via commands
     - Decode image files as `Image` objects in browser
     - Render all elements (page, images, guides) on `<canvas>`
     - **Pros:** Minimal IPC, native browser performance
     - **Cons:** More JS logic, image decoding in browser
   
   - **Option B: Rust-Rendered Bitmaps**
     - Call `render_canvas_preview(layout, zoom)` command
     - Rust generates full scene as RGBA bitmap
     - Display bitmap on canvas with `drawImage()`
     - **Pros:** Reuse existing canvas_widget logic
     - **Cons:** Heavy IPC, potential latency on large canvases
   
   - **Recommendation:** Prototype both, benchmark with realistic layouts

3. **Implement pan/zoom**
   - Track mouse wheel events for zoom level adjustment
   - Apply CSS `transform: scale()` to canvas wrapper, or
   - Use canvas transform matrix for rendering

4. **Implement drag interactions**
   - Mouse down on image → enter drag mode
   - Track mouse move deltas, debounce command calls
   - Call `update_image_position(id, x, y)` on mouse up
   - Optimistic updates: move image client-side immediately, confirm with backend

5. **Implement resize handles**
   - Render 8 handles (4 corners, 4 edges) as overlay elements
   - Detect handle clicks, enter resize mode
   - Track drag deltas, apply aspect ratio lock if enabled
   - Call `update_image_size(id, w, h)` on mouse up

### 4.4 State Management (state.js)
**Objectives:**
- Maintain local copy of layout state for fast UI updates
- Sync with backend on user actions
- Handle async command responses

**Tasks:**
1. **Define state object**
   - Mirror backend `AppState` structure
   - Include: layout, printers, printer_capabilities, ui_state

2. **Implement command wrappers (api.js)**
   - Wrap all Tauri `invoke()` calls in typed functions
   - Handle promise resolution/rejection
   - Update local state on success

3. **Event listeners**
   - Subscribe to Rust events (`print-progress`, `auto-save-complete`)
   - Update UI indicators (progress bars, status messages)

### 4.5 UI Component Implementation
**Build each section iteratively:**

1. **Top Bar (Printer & File Operations)**
   - Printer dropdown: populate from `discover_printers()` on load
   - File buttons: New, Open, Recent (dropdown), Save, Save As
   - Wire click handlers to call respective commands

2. **Toolbar (Image & Zoom Controls)**
   - Add Image button: trigger file dialog, call `add_images()`
   - Delete Image button: enabled only when image selected
   - Zoom buttons: +, -, Reset, Fit
   - Orientation toggle: Portrait/Landscape icon button

3. **Thumbnail Panel**
   - Fetch thumbnails via `get_image_thumbnail(path)`
   - Display as scrollable horizontal row
   - Click to select image (highlight, call `select_image()`)
   - Show image filename below thumbnail

4. **Canvas Area**
   - Render canvas with layout
   - Overlay rulers (SVG or CSS positioned divs with tick marks)
   - Selection outline on selected image
   - Resize handles when image selected

5. **Settings Panel (Tabbed)**
   - Tab buttons: Print Settings, Layout, Image Tools
   - **Print Settings Tab:**
     - Paper size dropdown (populate from PaperSize enum)
     - Borderless checkbox
     - Copies input
     - CUPS options (dynamic based on printer_capabilities)
   - **Layout Tab:**
     - Margin inputs (Top, Bottom, Left, Right in mm)
     - Page info display (dimensions, orientation)
   - **Image Tools Tab:**
     - Rotate buttons (CW, CCW)
     - Flip buttons (Horizontal, Vertical)
     - Size inputs (Width, Height in mm)
     - Aspect ratio lock checkbox
     - Opacity slider (0-100%)

6. **Status Bar**
   - Zoom percentage display (left side)
   - Optional: image count, selected image info

7. **Modal Dialogs**
   - Print progress dialog (rendering, sending, completed, failed)
   - Auto-save recovery prompt on startup
   - Error message popups

---

## 5. Integration & Testing Strategy

### 5.1 Incremental Integration Approach
**Phase 0: Canvas Rendering Prototype (3-5 days, pre-Week 1)**
- Build minimal prototypes for Pure Web vs Rust Bitmap rendering
- Benchmark performance with realistic layouts
- Make architectural decision on rendering strategy
- Document findings and update Section 4.3

**Phase 1: Scaffolding (Week 1)**
- Initialize Tauri project structure
- Comment out Iced dependency, audit dependent code
- Implement printer discovery command + web UI dropdown
- Test CUPS integration in Tauri context
- Validate IPC communication works
- Implement Tauri asset protocol for image serving

**Phase 2: Layout Core (Week 2)**
- Implement layout commands (paper size, margins, orientation)
- Build Print Settings and Layout tabs
- Test state synchronization

**Phase 3: Image Management (Week 2-3)**
- Implement image add/remove/move commands
- Build thumbnail panel and canvas rendering
- Test drag-and-drop interactions

**Phase 4: Image Manipulation (Week 3)**
- Implement rotate/flip/resize/opacity commands
- Build Image Tools tab
- Test transform caching

**Phase 5: Printing (Week 4)**
- Implement CUPS integration commands
- Build print progress modal
- Test full print workflow

**Phase 6: Persistence (Week 4)**
- Implement save/load/auto-save commands
- Add `.pxl` v0.2.1 compatibility test
- Add initialization cleanup check for legacy config
- Test file operations, recovery dialog

**Phase 7: Polish & Optimization (Week 5)**
- Performance profiling (canvas rendering latency)
- CSS refinements for Canon PPL aesthetic (80% similarity target)
- Validate HiDPI support on 4K display
- Keyboard shortcuts (global registration in tauri.conf.json)
- Accessibility fixes

### 5.2 Testing Checklist
**Pre-Implementation Tests (Phase 0):**
- [ ] Canvas rendering performance meets targets (<16ms frame time)
- [ ] Pure Web vs Rust Bitmap comparison documented
- [ ] Image caching strategy validated

**Backend Integration Tests (Week 1):**
- [ ] CUPS commands work in Tauri context (lpstat, lpoptions, lp)
- [ ] Tauri asset protocol serves images correctly
- [ ] Legacy config cleanup prompt triggers correctly

**Functional Tests (Manual):**
- [ ] Printer discovery lists all available CUPS printers
- [ ] CUPS capabilities load and populate dynamic dropdowns
- [ ] Paper size changes update canvas dimensions
- [ ] Margin inputs validate and apply correctly
- [ ] Orientation toggle swaps width/height
- [ ] Borderless mode resets margins
- [ ] Add images via file dialog
- [ ] Drag images to new positions
- [ ] Resize images with 8 handles
- [ ] Aspect ratio lock works during resize
- [ ] Rotate 90° CW/CCW updates dimensions
- [ ] Flip horizontal/vertical updates preview
- [ ] Opacity slider updates transparency
- [ ] Thumbnail selection syncs with canvas
- [ ] Print job executes with correct settings
- [ ] Save/load project preserves all state
- [ ] v0.2.1 .pxl files load correctly in v0.3.0
- [ ] Auto-save triggers every 30 seconds
- [ ] Recovery dialog appears on startup with orphaned auto-save
- [ ] Recent files menu loads and opens projects
- [ ] HiDPI scaling works correctly on 4K displays
- [ ] Global keyboard shortcuts trigger commands

**Performance Benchmarks:**
- Canvas rendering latency: <16ms per frame at 100% zoom
- Image add latency: <500ms for 10MB file
- Print render: <5 seconds for A4 with 5 images at 300 DPI
- Project save/load: <1 second for 50-image layout

**Cross-Platform Tests:**
- Fedora 40+: Primary target, test RPM packaging
- Ubuntu 24.04: Secondary target
- RHEL 9: Ensure CUPS compatibility

### 5.3 Validation Criteria (Feature Parity Checklist)
Compare Tauri UI against current Iced implementation:
- [ ] All 50+ paper sizes available
- [ ] All CUPS options dynamically loaded
- [ ] Image transforms (rotate, flip, opacity) produce identical results
- [ ] Print output matches current 300 DPI quality
- [ ] Project files load/save without data loss
- [ ] Auto-save/recovery works identically
- [ ] Performance meets or exceeds current Iced build

---

## 6. Packaging & Deployment Updates

### 6.1 CI/CD Workflow Changes
**Update `.github/workflows/release-rpm.yml`:**

**New Steps Required:**
1. **Install Node LTS**
   - Add `actions/setup-node@v4` before build
   - Specify Node version (e.g., 20.x)

2. **Install npm dependencies**
   - Run `npm ci` (or `npm install` if no lockfile)
   - Ensure all frontend deps installed

3. **Build frontend assets**
   - Run `npm run build` to generate static bundle
   - Output should go to `src-tauri/dist/` or configured output dir

4. **Install Tauri CLI**
   - Add `cargo install tauri-cli` or use `cargo tauri` from workspace

5. **Build Tauri application**
   - Run `cargo tauri build --bundles none`
   - This compiles Rust + bundles frontend assets into single binary

6. **Package RPM**
   - Update `packaging/rpm/print-layout.spec` to handle Tauri binary
   - Copy `src-tauri/target/release/print-layout` to RPM build location
   - Adjust paths if needed (frontend assets are embedded in binary)

7. **Upload artifacts**
   - Continue uploading RPM and SRPM to GitHub releases

**Dependencies to Install on CI Runners:**
- Node 22.x LTS (via actions/setup-node@v4 with explicit version)
- WebKitGTK development packages (`webkit2gtk-4.1-devel` on Fedora)
- Existing: Rust, rpmbuild, CUPS dev libs

**Critical CI Testing (Week 8 Priority):**
- Test "Hello World" Tauri build in CI first (before Week 1)
- Verify RPM packaging with embedded assets
- Validate on clean Fedora VM
- This prevents discovering packaging issues late in development

### 6.2 RPM Spec File Updates
**Modify `packaging/rpm/print-layout.spec`:**

**BuildRequires additions:**
- `nodejs` (or specify version)
- `npm`
- `webkit2gtk4.1-devel` (Tauri's WebView dependency)

**Build section:**
```
%build
cd %{_builddir}/PrintLayout-%{version}
npm ci
npm run build
cargo tauri build --bundles none
```

**Install section:**
- Copy `src-tauri/target/release/print-layout` to `%{buildroot}%{_bindir}/`
- Desktop file and icons remain unchanged

**Files section:**
- No additional files (frontend assets embedded in binary)

### 6.3 Local Development Workflow
**New commands for developers:**
- `npm run dev` - Start Tauri in development mode with hot reload
- `npm run build` - Build production frontend assets
- `cargo tauri dev` - Alternative dev command
- `cargo tauri build` - Build production binary

**Documentation updates:**
- Update `README.md` with new dependencies (Node, WebKitGTK)
- Add `CONTRIBUTING.md` section on frontend development
- Document how to switch between Iced and Tauri builds (if feature flag used)

---

## 7. Risk Mitigation & Contingency Plans

### 7.1 Identified Risks

**Risk 1: Canvas Performance Below Expectations**
- **Likelihood:** Medium
- **Impact:** High (core UX degradation)
- **Mitigation:**
  - Prototype both WebGL and Canvas2D early (Week 1)
  - Benchmark with realistic layouts (10+ images, high-res)
  - Fallback: Use Rust-rendered bitmaps streamed via IPC
  - Worst case: Retain Iced for canvas, use Tauri for settings panels only

**Risk 2: WebView Memory Usage Excessive**
- **Likelihood:** Low-Medium
- **Impact:** Medium (system resource concerns)
- **Mitigation:**
  - Profile memory with Valgrind/Heaptrack during development
  - Lazy-load images in thumbnails (load on-demand)
  - Implement image cache eviction policy
  - Document minimum system requirements

**Risk 3: CUPS Integration Breaks on Tauri IPC**
- **Likelihood:** Low
- **Impact:** High (printing unusable)
- **Mitigation:**
  - Test CUPS commands in isolated Tauri command early (Week 1)
  - Maintain existing subprocess approach (no changes to CUPS calls)
  - Add integration tests for printer discovery/capabilities

**Risk 4: State Synchronization Bugs**
- **Likelihood:** Medium
- **Impact:** Medium (data loss, UI inconsistencies)
- **Mitigation:**
  - Use single source of truth (Rust-managed state)
  - All state mutations via commands (no direct JS state changes)
  - Add verbose logging for state transitions
  - Implement state diffing for auto-save

**Risk 5: Tooling Learning Curve (Node/NPM)**
- **Likelihood:** Low (team has minimal Node experience)
- **Impact:** Low (slows initial setup)
- **Mitigation:**
  - Use vanilla setup (no complex bundlers initially)
  - Document all npm commands in `CONTRIBUTING.md`
  - Pair with frontend-experienced developer if available

**Risk 6: Packaging Complexity on CI**
- **Likelihood:** Medium
- **Impact:** Medium (release pipeline delays)
- **Mitigation:**
  - Test CI workflow in feature branch before merge
  - Add manual fallback instructions for local RPM builds
  - Keep Iced build pipeline functional until Tauri stabilizes

### 7.2 Rollback Strategy
**Hard Switch Approach (Chosen):**
- Remove Iced dependency entirely in `gui_rework` branch
- `main` branch retains stable v0.2.1 Iced implementation
- Separate branch eliminates need for feature flags
- Rollback = revert to `main` branch if needed
- Benefits: Faster development, cleaner codebase, no dual-UI maintenance

**If Tauri migration fails or stalls:**
1. `main` branch remains stable (v0.2.1)
2. Continue Tauri work in `gui_rework` without merge pressure
3. Option to cherry-pick non-UI improvements to `main`
4. Communicate timeline adjustments transparently

---

## 8. Timeline & Resource Allocation

### 8.1 Estimated Timeline (8 weeks total)

**Pre-Week 1: Phase 0 (3-5 days)**
- Build canvas rendering prototypes (Pure Web vs Rust Bitmap)
- Benchmark performance with 10+ image layouts
- Make architectural decision
- Document findings
- **Deliverable:** Rendering strategy decision document

**Week 1: Preparation & Architecture**
- Extract core modules into workspace crates
- Define DTO layer
- Initialize Tauri project
- **Comment out Iced dependency, audit dependent code**
- Implement printer discovery + CUPS integration test
- Implement Tauri asset protocol for images
- Test "Hello World" Tauri build in CI
- **Deliverable:** Tauri app displays printer dropdown, images load via asset protocol

**Week 2: Backend Command Layer**
- Implement layout commands (paper size, margins, orientation)
- Implement image commands (add, remove, move, resize)
- Implement state management (Mutex-wrapped AppState)
- **Deliverable:** All commands testable via Tauri DevTools

**Week 3: Frontend Scaffold & Canvas**
- Build HTML structure and CSS styling (Canon PPL inspired, 80% similarity)
- Implement canvas rendering (using Phase 0 decision)
- Implement HiDPI support (devicePixelRatio scaling)
- Build thumbnail panel with asset protocol
- Wire up layout and image commands to UI
- **Deliverable:** Functional canvas with drag-and-drop, HiDPI-ready

**Week 4: Image Manipulation & Printing**
- Implement image transform commands (rotate, flip, opacity)
- Build Image Tools tab
- Implement printing commands
- Build print progress modal
- **Deliverable:** Full printing workflow functional

**Week 5: Persistence & File Operations**
- Implement save/load/auto-save commands
- Add .pxl v0.2.1 compatibility test
- Add legacy config cleanup on initialization
- Build file operation dialogs (Tauri dialog API)
- Implement recovery dialog
- **Deliverable:** Project files work end-to-end, v0.2.1 files load correctly

**Week 6: Polish & Optimization**
- CSS refinements (Canon PPL aesthetic, 80% target)
- Global keyboard shortcuts (tauri.conf.json)
- Validate HiDPI on 4K display
- Accessibility improvements (ARIA labels, focus management)
- Performance profiling and optimization
- Consolidate image caches (3 → 2)
- **Deliverable:** UI visually polished, performant, HiDPI-tested

**Week 7: Testing & Bug Fixes**
- Execute full testing checklist
- Fix identified bugs
- Cross-platform validation
- **Deliverable:** All tests passing, bugs triaged

**Week 8: Packaging & Documentation**
- Update CI/CD workflows
- Update RPM spec file
- Test GitHub Actions release pipeline
- Update all documentation (README, INSTALL, USAGE)
- **Deliverable:** Beta release ready (v0.3.0-beta.1)

### 8.2 Resource Requirements
**Personnel:**
- **1 Rust Engineer** (backend refactor, Tauri commands): 50-60 hours
- **1 Frontend Developer** (HTML/CSS/JS, canvas rendering): 50-60 hours
- **Shared QA/Testing** (manual testing, bug verification): 20 hours
- **DevOps** (CI pipeline updates, packaging): 10 hours

**Infrastructure:**
- CI runner time: Additional 30 minutes per build (Node install + npm)
- Development machines: Must have Node LTS installed
- Build agents: Add WebKitGTK-devel packages

**Optional:**
- Design consultation for Canon PPL theme accuracy (screenshots, color matching)
- Performance profiling tools (Lighthouse, Chrome DevTools)

---

## 9. Success Criteria & Acceptance

### 9.1 Definition of Done
**v0.3.0 Release Ready When:**
1. All 47 current message handlers have equivalent Tauri commands
2. UI feature parity checklist 100% complete
3. Performance benchmarks meet targets (<16ms canvas, <5s print render)
4. Zero data loss in save/load/auto-save workflows
5. CI pipeline produces RPM successfully
6. Documentation updated (README, INSTALL, CONTRIBUTING)
7. Beta testing feedback incorporated (if applicable)

### 9.2 Beta Release Criteria (v0.3.0-beta.1)
**Before Public Beta:**
- Core workflows functional (add images, adjust settings, print, save/load)
- Known bugs documented in GitHub issues
- Stability: No crashes in 30-minute usage session
- Beta disclaimer in UI and release notes

**Feedback Collection:**
- GitHub Discussions for beta feedback
- Track performance reports (canvas lag, print quality)
- Monitor memory usage reports

### 9.3 Go/No-Go Decision Points
**End of Week 4 Checkpoint:**
- Evaluate canvas performance: If >50ms latency, pivot to Rust-rendered bitmaps
- Assess state synchronization stability: If >5 critical bugs, extend timeline

**End of Week 7 Checkpoint:**
- Feature parity: If <90% complete, delay release
- Performance: If targets not met, identify optimization sprint

**Final Release Decision:**
- All acceptance tests passing
- No P0/P1 bugs in backlog
- Positive beta feedback (if conducted)
- Maintainer approval

---

## 10. Post-Migration Roadmap

### 10.1 Future Enhancements Enabled by Tauri
**Now Easier with Web Stack:**
1. **Rich Animations** - CSS transitions for panel slides, modal fades
2. **Advanced Canvas Tools** - Layer blending modes, filters, effects
3. **Theming System** - User-customizable color schemes via CSS variables
4. **Localization (i18n)** - Structured strings in JSON, easy translation
5. **Plugin System** - Load external scripts for custom image processing

### 10.2 Deferred Features (Not in v0.3.0)
**From original upgrade_plan.md (now delayed):**
- Multi-page support (v0.4.0 target)
- PDF export (v0.4.0 target)
- Undo/Redo system (v0.5.0 target)

**Rationale:** Focus entirely on UI migration first, add features after stabilization.

### 10.3 Long-Term Vision
- **Cross-platform expansion:** macOS/Windows builds (Tauri makes this easier)
- **Cloud sync:** Optional project sync via backend service
- **Print queue management:** View/cancel jobs from UI
- **Template library:** Pre-made layouts for common use cases (greeting cards, portfolios)

---

## 11. Next Steps & Action Items

### 11.1 Immediate Actions (This Week)
1. **Approve this upgrade plan** - Team review and sign-off
2. **Gather Canon PPL design references**
   - Collect screenshots (layout, colors, spacing, typography)
   - Extract color palette and spacing values
   - Document 80% similarity targets (not pixel-perfect)
3. **Set up development environment**
   - Install Node 22.x LTS on all dev machines
   - Install WebKitGTK-devel packages
4. **Phase 0 Preparation**
   - Set up minimal Tauri project for prototypes
   - Prepare test layouts with 5-10 images
5. **Test CI pipeline**
   - Verify "Hello World" Tauri build in GitHub Actions
   - Confirm Node 22.x, WebKitGTK packages install correctly

### 11.2 Phase 0 Tasks (Pre-Week 1)
1. Build Pure Web Rendering prototype
2. Build Rust Bitmap Rendering prototype
3. Benchmark both with 10-image layouts
4. Measure frame times, memory, IPC overhead
5. Document findings and make decision
6. Update Section 4.3 with chosen implementation

### 11.3 Week 1 Kickoff Tasks
1. Comment out Iced in Cargo.toml, identify dependent code
2. Extract `layout`, `printing`, `config` into workspace crates
3. Define first DTO (LayoutDTO) and implement conversions
4. Implement `discover_printers()` and test CUPS in Tauri
5. Implement Tauri asset protocol for image serving
6. Create basic HTML structure with printer dropdown
7. Validate end-to-end IPC communication

### 11.3 Communication Plan
- **Weekly standup:** Progress updates, blocker discussion
- **GitHub issues:** Track bugs and feature tasks
- **Documentation:** Update wiki/docs as architecture solidifies
- **User communication:** Announce GUI rework in progress, set expectations for v0.3.0 timeline

---

## Appendix A: Command Reference Table

| Category | Command Name | Parameters | Returns | Priority |
|----------|-------------|------------|---------|----------|
| Printer | `discover_printers` | - | `Vec<PrinterInfoDTO>` | P0 |
| Printer | `get_printer_capabilities` | `printer: String` | `PrinterCapabilitiesDTO` | P0 |
| Printer | `execute_print_job` | `job: PrintJobDTO` | `Result<String, String>` | P0 |
| Layout | `new_layout` | - | `LayoutDTO` | P0 |
| Layout | `update_paper_size` | `size: PaperSizeDTO` | `()` | P0 |
| Layout | `update_margins` | `top, bottom, left, right: f32` | `()` | P0 |
| Layout | `toggle_orientation` | - | `LayoutDTO` | P0 |
| Layout | `toggle_borderless` | `enabled: bool` | `()` | P0 |
| Image | `add_images` | `paths: Vec<String>` | `Vec<PlacedImageDTO>` | P0 |
| Image | `remove_image` | `id: String` | `()` | P0 |
| Image | `update_image_position` | `id: String, x: f32, y: f32` | `()` | P0 |
| Image | `update_image_size` | `id: String, w: f32, h: f32` | `()` | P0 |
| Image | `rotate_image` | `id: String, degrees: f32` | `()` | P1 |
| Image | `flip_image` | `id: String, h: bool, v: bool` | `()` | P1 |
| Image | `update_image_opacity` | `id: String, opacity: f32` | `()` | P1 |
| Image | `select_image` | `id: String` | `()` | P0 |
| File | `save_project` | `path: String` | `Result<(), String>` | P0 |
| File | `load_project` | `path: String` | `Result<ProjectLayoutDTO, String>` | P0 |
| File | `get_recent_files` | - | `Vec<String>` | P1 |
| File | `auto_save` | - | `Result<(), String>` | P1 |
| File | `check_recovery_file` | - | `Option<String>` | P1 |
| Render | `render_canvas_preview` | `layout: LayoutDTO, zoom: f32` | `Vec<u8>` | P0/P1 |
| Render | `get_image_thumbnail` | `path: String` | `Vec<u8>` | P1 |
| Config | `load_preferences` | - | `UserPreferencesDTO` | P1 |
| Config | `save_preferences` | `prefs: UserPreferencesDTO` | `()` | P1 |

**Priority Legend:**
- P0: Critical (blocks core functionality)
- P1: High (important but has workarounds)
- P2: Medium (nice-to-have)

---

## Appendix B: UI Component Mapping

| Current Iced Component | Tauri Web Equivalent | Implementation Notes |
|------------------------|----------------------|----------------------|
| `pick_list` (printer) | `<select>` dropdown | Populate via `discover_printers()` |
| `button` (Add Image) | `<button>` + Tauri dialog | `rfd::AsyncFileDialog` already in crates |
| `text_input` (margins) | `<input type="number">` | Validate on blur, call `update_margins()` |
| `canvas` widget | `<canvas>` + WebGL/2D | See rendering strategy decision |
| `scrollable` (thumbnails) | `<div>` with `overflow-x: scroll` | CSS flexbox layout |
| `row`/`column` | Flexbox/Grid CSS | Standard web layout |
| `container` | `<div>` with CSS | Custom styling per section |
| `opaque` modal overlay | `<div>` with backdrop | CSS `rgba(0,0,0,0.5)` background |
| `progress_bar` | `<progress>` element | Update via Tauri events |
| `checkbox` | `<input type="checkbox">` | Standard HTML |
| `pick_list` (CUPS options) | `<select>` populated dynamically | Fetch via `get_printer_capabilities()` |

---

## Appendix C: Glossary

- **DTO (Data Transfer Object):** Serializable struct for passing data over IPC
- **IPC (Inter-Process Communication):** Mechanism for Rust ↔ Web communication in Tauri
- **MVU (Model-View-Update):** Iced's architecture pattern (Elm-inspired)
- **CUPS:** Common Unix Printing System (Linux printer API)
- **XDG:** Cross-Desktop Group (Linux standards for config paths)
- **WebView:** System-provided browser engine (WebKitGTK on Linux)
- **Canvas2D:** HTML5 2D rendering context
- **WebGL:** Hardware-accelerated 3D graphics API in browsers
- **DevicePixelRatio:** Ratio of physical pixels to CSS pixels (for HiDPI displays)

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
