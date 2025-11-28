# Canvas Image Rendering Research - Iced 0.12

## Question
How to render actual images on an Iced canvas widget in Rust using Iced 0.12?

## Findings

### TL;DR
**`Frame::draw_image()` does NOT exist in the public API of Iced 0.12's `iced::widget::canvas::Frame`**, even with the "image" feature enabled.

### Detailed Investigation

1. **Feature Requirements**
   - Added `image` feature to iced in Cargo.toml: `iced = { version = "0.12", features = ["canvas", "image", "tokio", "debug"] }`
   - This enables image support for `iced::widget::Image` widgets, but NOT for canvas Frame

2. **Frame API Limitations**
   - The `iced::widget::canvas::Frame` type is a thin wrapper around `geometry::Frame<Renderer>`
   - While `geometry::frame::Backend` trait (internal) has `draw_image()` method (guarded by `#[cfg(feature = "image")]`)
   - This method is NOT exposed in the public `iced::widget::canvas::Frame` API

3. **Source Code Evidence**
   From `graphics/src/geometry/frame.rs`:
   ```rust
   /// Draws the given [`Image`] on the [`Frame`] inside the given bounds.
   #[cfg(feature = "image")]
   pub fn draw_image(&mut self, bounds: Rectangle, image: impl Into<Image>) {
       self.raw.draw_image(bounds, image);
   }
   ```
   
   However, this method exists on `geometry::Frame<Renderer>` in the graphics crate, not on `iced::widget::canvas::Frame`.

4. **Tested Approaches** (All Failed)
   - ❌ `frame.draw_image(bounds, handle)` - method not found
   - ❌ `frame.draw_image(bounds, image::Image { ... })` - method not found  
   - ❌ `frame.draw_image(bounds, canvas::Image::new(handle))` - `canvas::Image` doesn't exist
   - ❌ Using `iced::advanced::image` - module is private without "advanced" feature
   - ❌ Using `iced::core::image` - `core` module is private

## Alternative Solutions

### Option 1: Use Separate Image Widgets (RECOMMENDED)
Instead of rendering images on canvas, use `iced::widget::Image` widgets positioned absolutely:

```rust
use iced::widget::{canvas, container, image, stack};

// In your view function:
stack([
    container(your_canvas_widget).into(),
    container(image(handle))
        .style(|_theme| container::Style {
            // Position absolutely at x, y
            ...
        })
        .width(width)
        .height(height)
        .into(),
])
```

### Option 2: Draw Placeholder Rectangles (Current Implementation)
Keep using placeholder colored rectangles on canvas as visual representations:

```rust
let image_rect = Path::rectangle(Point::new(x, y), Size::new(width, height));
frame.fill(&image_rect, Color::from_rgb(0.9, 0.9, 1.0));
frame.stroke(&image_rect, Stroke::default().with_width(1.0).with_color(Color::BLACK));
```

### Option 3: Wait for Future Iced Version
Monitor https://github.com/iced-rs/iced for when `draw_image` becomes available in the public canvas Frame API.

### Option 4: Custom Renderer
Implement your own geometry backend that extends the Frame functionality, but this requires deep integration with Iced internals.

## Recommendation

For a print layout application, **Option 1 (separate Image widgets)** is the best approach because:
- Images will render at full quality
- You get proper image loading/caching from Iced
- You can still use canvas for guidelines, grids, selection rectangles, etc.
- Layering with `stack` widget allows proper z-ordering

## Implementation Strategy

1. Keep canvas for:
   - Page background and margins
   - Grid lines
   - Selection highlights
   - Resize handles
   - Guidelines

2. Use separate `Image` widgets for:
   - Actual photo rendering
   - Position them using absolute positioning in a `stack` widget
   - Transform coordinates from mm to pixels for positioning

This approach separates concerns and uses each widget for its strength.
