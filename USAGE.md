# Print Layout Usage Guide

This guide covers all features of Print Layout and provides tutorials for common workflows.

## Table of Contents

1. [Quick Start](#quick-start)
2. [User Interface Overview](#user-interface-overview)
3. [Working with Images](#working-with-images)
4. [Page Setup](#page-setup)
5. [Printing](#printing)
6. [Project Management](#project-management)
7. [Keyboard Shortcuts](#keyboard-shortcuts)
8. [Tips and Tricks](#tips-and-tricks)
9. [FAQ](#faq)

---

## Quick Start

### Creating Your First Print Layout

1. **Launch Print Layout**
   ```
   print-layout
   ```

2. **Add Images**
   - Click "Add Image" in the top toolbar
   - Select one or more images from the file browser
   - Images appear in the thumbnail panel at the bottom

3. **Arrange Images**
   - Click an image thumbnail to select it
   - The image appears on the canvas
   - Drag the image to position it
   - Use resize handles to adjust size

4. **Configure Page**
   - Select paper size from the "Layout" tab
   - Set margins as needed
   - Toggle between Portrait/Landscape

5. **Print**
   - Select your printer from the dropdown
   - Choose print quality
   - Click "Print"

---

## User Interface Overview

### Main Window Layout

```
┌─────────────────────────────────────────────────────────────────┐
│  Add Image  │  Zoom In/Out  │  Reset  │  Orientation Toggle     │
├─────────────┬───────────────────────────────────────────────────┤
│             │                                                   │
│   Settings  │              Canvas Preview                       │
│    Panel    │                                                   │
│             │         (Paper with images)                       │
│  [Tabs:]    │                                                   │
│  - Print    │                                                   │
│  - Layout   │                                                   │
│  - Color    │                                                   │
│  - Image    │                                                   │
│             │                                                   │
├─────────────┴───────────────────────────────────────────────────┤
│                    Thumbnail Panel                              │
│   [img1] [img2] [img3] ...                                      │
└─────────────────────────────────────────────────────────────────┘
```

### Settings Panel Tabs

#### Print Settings Tab
- Printer selection dropdown
- Print quality (Highest, High, Standard, Draft)
- Print button

#### Layout Tab
- Paper size selection
- Paper type (Plain, Glossy, Matte, etc.)
- Margins (Top, Bottom, Left, Right)
- Borderless printing toggle

#### Color Tab
- Color mode selection
- ICC profile options
- Black and white conversion

#### Image Tools Tab
- Rotation controls (90° CW/CCW)
- Flip controls (Horizontal/Vertical)
- Width/Height inputs with aspect ratio lock
- Opacity slider

---

## Working with Images

### Adding Images

**Via Toolbar:**
1. Click "Add Image" button
2. Navigate to your images
3. Select one or more files
4. Click "Open"

**Supported Formats:**
- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- BMP (.bmp)
- WebP (.webp)

### Selecting Images

- **Single click** on a thumbnail to select
- Selected image shows blue border on canvas
- Only one image can be selected at a time

### Positioning Images

1. Select an image
2. Click and drag anywhere on the image
3. Release to place at new position

**Tip:** The canvas shows the printable area. White area is your paper, gray area is outside.

### Resizing Images

1. Select an image
2. Hover over any edge or corner to see resize cursor
3. Click and drag to resize

**Resize Handles:**
- **Corners:** Resize both width and height
- **Top/Bottom edges:** Resize height only
- **Left/Right edges:** Resize width only

**Using Exact Dimensions:**
1. Go to "Image Tools" tab
2. Enter width or height in mm
3. Enable "Lock Aspect Ratio" to maintain proportions
4. Press Enter or click elsewhere to apply

### Rotating Images

**90° Clockwise:**
- Click "CW" button in Image Tools tab
- Or press `R` key

**90° Counter-clockwise:**
- Click "CCW" button in Image Tools tab
- Or press `Shift+R`

Rotations accumulate (4 rotations = back to original)

### Flipping Images

**Horizontal Flip (Mirror):**
- Click "H Flip" button in Image Tools tab
- Creates a left-right mirror

**Vertical Flip:**
- Click "V Flip" button in Image Tools tab
- Creates a top-bottom mirror

### Adjusting Opacity

1. Select an image
2. Go to "Image Tools" tab
3. Drag the Opacity slider (0-100%)
4. Preview updates in real-time

**Use Cases:**
- Create watermark effects
- Layer multiple images
- Artistic compositions

### Deleting Images

1. Select the image
2. Press `Delete` or `Backspace` key
3. Image is removed from layout

---

## Page Setup

### Paper Size

1. Go to "Layout" tab
2. Click paper size dropdown
3. Select from available sizes:

**Standard (A-series):**
- A0 (841 × 1189 mm) through A10 (26 × 37 mm)

**Standard (B-series):**
- B0 (1000 × 1414 mm) through B10 (31 × 44 mm)

**North American:**
- Letter (8.5 × 11")
- Legal (8.5 × 14")
- Tabloid (11 × 17")
- Ledger (17 × 11")

**Photo Paper:**
- 3.5×5", 4×6", 5×5", 5×7"
- 7×10", 8×10", 10×12"
- 11×17", 12×12", 13×19"

### Paper Type

Select paper type to optimize print settings:
- **Plain Paper:** Standard office paper
- **Super High Gloss:** Premium photo paper with mirror finish
- **Glossy:** Photo paper with shiny finish
- **Semi-Gloss:** Photo paper with slight sheen
- **Matte:** Non-reflective finish
- **Fine Art:** Textured art papers

### Margins

Set margins in millimeters:
1. Enter values in Top, Bottom, Left, Right fields
2. Press Enter to apply
3. Canvas updates to show printable area

**Borderless Printing:**
- Enable "Borderless" checkbox
- Sets all margins to 0
- Requires printer support

### Orientation

**Portrait:** Taller than wide
**Landscape:** Wider than tall

Toggle using:
- Toolbar orientation button
- Keyboard shortcut (if available)

---

## Printing

### Selecting a Printer

1. Go to "Print Settings" tab
2. Click printer dropdown
3. Select your printer
4. Printer is remembered for next session

**Note:** Only CUPS-compatible printers are shown.

### Print Quality

Choose quality based on your needs:

| Quality | DPI | Use Case |
|---------|-----|----------|
| Highest | 300 | Final prints, photos |
| High | 300 | Quality prints |
| Standard | 300 | Everyday printing |
| Draft | 300 | Test prints, proofs |

### Color Settings

**Use ICC Profile:** Use embedded color profiles for accurate colors
**Driver Matching:** Let printer driver handle color
**No Color Correction:** Print raw colors
**Black and White:** Convert to grayscale

### Print Process

1. Configure all settings
2. Click "Print" button
3. Wait for status message
4. Check printer for output

---

## Project Management

### Saving Projects

**Manual Save:**
- Press `Ctrl+S`
- Or use File menu (if available)
- Choose location and filename
- Files saved as `.pxl` format

**Auto-Save:**
- Enabled by default
- Saves every 30 seconds when modified
- Location: `~/.local/share/print-layout/autosave/`

### Loading Projects

**Open File:**
- Press `Ctrl+O`
- Select `.pxl` file
- Layout is restored

**Recent Files:**
- Access from File menu
- Shows up to 10 recent projects
- Click to open

### Auto-Save Recovery

If the application was closed unexpectedly:
1. On next launch, recovery dialog appears
2. Choose "Recover" to restore auto-save
3. Choose "Discard" to start fresh

### Backups

- Automatic backups created on save
- Location: `~/.local/share/print-layout/backups/`
- Keeps 5 most recent versions
- Named with timestamp for easy identification

---

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Add Image | `Ctrl+I` |
| Save Project | `Ctrl+S` |
| Open Project | `Ctrl+O` |
| Zoom In | `Ctrl++` or `Ctrl+=` |
| Zoom Out | `Ctrl+-` |
| Reset Zoom | `Ctrl+0` |
| Rotate CW | `R` |
| Rotate CCW | `Shift+R` |
| Delete Image | `Delete` or `Backspace` |

---

## Tips and Tricks

### Getting Best Print Quality

1. **Use high-resolution images**
   - 300 DPI at print size is ideal
   - Larger is better than smaller

2. **Match paper type setting to actual paper**
   - Incorrect paper type = poor quality

3. **Let prints dry**
   - Especially on glossy paper
   - Handle by edges

### Efficient Workflows

1. **Set up defaults first**
   - Configure printer, paper size, margins
   - Settings are remembered

2. **Use aspect ratio lock**
   - Prevents accidental distortion
   - Enable in Image Tools tab

3. **Check with Draft quality first**
   - Save ink/toner on test prints
   - Switch to High/Highest for final

### Troubleshooting Common Issues

**Images look blurry:**
- Original image may be too small
- Try reducing size on canvas
- Use higher resolution source

**Colors don't match screen:**
- Try different color mode settings
- Calibrate your monitor
- Use ICC profiles if available

**Print is cut off:**
- Increase margins
- Check borderless setting
- Some printers can't print to edge

---

## FAQ

### Q: What file formats can I import?

A: PNG, JPEG, GIF, BMP, and WebP are supported. RAW and HEIC formats are not currently supported.

### Q: Can I print multiple copies?

A: Currently, you need to click Print multiple times. Batch printing is planned for a future release.

### Q: Where are my settings saved?

A: Settings are stored in `~/.config/print-layout/` following XDG standards.

### Q: Can I create multi-page layouts?

A: Not currently. Multi-page support is planned for a future release.

### Q: Why doesn't my printer appear?

A: Print Layout uses CUPS. Ensure your printer is configured in your system's printer settings.

### Q: Can I export to PDF?

A: PDF export is planned for a future release. Currently, you can only print directly.

### Q: Is there undo/redo?

A: Not currently. This feature is planned for a future release. Use auto-save recovery if you make a mistake.

### Q: How do I reset all settings?

A: Delete the config directory:
```bash
rm -rf ~/.config/print-layout/
```

### Q: Can I use this commercially?

A: Yes! Print Layout is open source under a permissive license. Check the LICENSE file for details.

---

## Getting Help

- **GitHub Issues:** Report bugs or request features
- **README:** Basic usage and installation
- **CHANGELOG:** What's new in each version

---

*Print Layout - Professional photo layout and printing for Linux*
