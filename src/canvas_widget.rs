// canvas_widget.rs - Canvas widget implementation with image rendering
// Updated for Iced 0.13 with draw_image support

use crate::layout::{Layout, PlacedImage};
use iced::mouse::{self, Cursor};
use iced::widget::canvas::{self, Cache, Frame, Geometry, Image, Path, Program, Stroke, Text};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

/// Cache key that includes transform parameters
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TransformKey {
    path: PathBuf,
    rotation_degrees: i32,  // Rounded to int for hash
    flip_horizontal: bool,
    flip_vertical: bool,
    opacity_percent: u8,    // 0-100 for hash
}

impl TransformKey {
    fn from_placed_image(img: &PlacedImage) -> Self {
        Self {
            path: img.path.clone(),
            rotation_degrees: (img.rotation_degrees as i32) % 360,
            flip_horizontal: img.flip_horizontal,
            flip_vertical: img.flip_vertical,
            opacity_percent: (img.opacity * 100.0) as u8,
        }
    }
}

/// Image handle cache to avoid recreating handles
#[derive(Debug, Default)]
pub struct ImageCache {
    cache: HashMap<TransformKey, iced::widget::image::Handle>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get or create a transformed image handle for the given placed image
    /// Uses source_cache to avoid reloading images from disk
    pub fn get_transformed_handle(
        &mut self, 
        img: &PlacedImage, 
        source_cache: &mut SourceImageCache
    ) -> Option<iced::widget::image::Handle> {
        let key = TransformKey::from_placed_image(img);
        
        if let Some(handle) = self.cache.get(&key) {
            return Some(handle.clone());
        }

        // Get source image from cache (or load it)
        let source = source_cache.get_or_load(&img.path)?;

        // Apply rotation (90° increments)
        let rotation_normalized = ((img.rotation_degrees % 360.0) + 360.0) % 360.0;
        let rotated = if rotation_normalized >= 85.0 && rotation_normalized <= 95.0 {
            source.rotate90()
        } else if rotation_normalized >= 175.0 && rotation_normalized <= 185.0 {
            source.rotate180()
        } else if rotation_normalized >= 265.0 && rotation_normalized <= 275.0 {
            source.rotate270()
        } else {
            source.clone()
        };

        // Apply flips
        let flipped = if img.flip_horizontal && img.flip_vertical {
            rotated.fliph().flipv()
        } else if img.flip_horizontal {
            rotated.fliph()
        } else if img.flip_vertical {
            rotated.flipv()
        } else {
            rotated
        };

        // Apply opacity
        let mut rgba = flipped.to_rgba8();
        if img.opacity < 1.0 {
            let opacity_factor = img.opacity.clamp(0.0, 1.0);
            for pixel in rgba.pixels_mut() {
                pixel[3] = (pixel[3] as f32 * opacity_factor) as u8;
            }
        }

        // Create handle from RGBA pixels
        let (width, height) = rgba.dimensions();
        let handle = iced::widget::image::Handle::from_rgba(
            width,
            height,
            rgba.into_raw(),
        );
        
        self.cache.insert(key, handle.clone());
        Some(handle)
    }
    
    /// Clear the cache (e.g., when images change)
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

/// Messages that can be sent from the canvas
#[derive(Debug, Clone)]
pub enum CanvasMessage {
    SelectImage(String),
    DeselectAll,
    ImageMoved(String, f32, f32),
    ImageResized(String, f32, f32),
    CanvasClicked(f32, f32),
    MouseMoved(f32, f32),
    MouseReleased,
    /// Start resizing from a specific handle
    StartResize(String, ResizeHandle),
}

/// Which resize handle is being dragged
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
}

/// Cache for source images loaded from disk (to avoid repeated disk I/O)
#[derive(Debug, Default)]
pub struct SourceImageCache {
    cache: HashMap<PathBuf, image::DynamicImage>,
}

impl SourceImageCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get or load a source image from disk
    pub fn get_or_load(&mut self, path: &PathBuf) -> Option<&image::DynamicImage> {
        if !self.cache.contains_key(path) {
            if path.exists() {
                if let Ok(img) = image::open(path) {
                    self.cache.insert(path.clone(), img);
                }
            }
        }
        self.cache.get(path)
    }

    /// Remove an image from cache
    #[allow(dead_code)]
    pub fn remove(&mut self, path: &PathBuf) {
        self.cache.remove(path);
    }

    /// Clear the entire cache
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

/// The canvas widget for displaying and interacting with the layout
pub struct LayoutCanvas {
    layout: Layout,
    zoom: f32,
    cache: Cache,
    // Use RefCell for interior mutability to allow caching in draw()
    image_cache: RefCell<ImageCache>,
    // Cache for source images loaded from disk
    source_cache: RefCell<SourceImageCache>,
}

impl LayoutCanvas {
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            zoom: 1.0,
            cache: Cache::new(),
            image_cache: RefCell::new(ImageCache::new()),
            source_cache: RefCell::new(SourceImageCache::new()),
        }
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.cache.clear();
    }

    /// Update layout without clearing the render cache - for position/size changes during drag
    /// This is more efficient for interactive operations where only positions change
    #[allow(dead_code)]
    pub fn update_layout_positions(&mut self, layout: Layout) {
        self.layout = layout;
        // Don't clear cache - positions are handled differently
        // The cache will be invalidated naturally when needed
        self.cache.clear(); // Still need to clear for now since positions affect rendering
    }

    /// Update just the selected image's position without full layout update
    pub fn update_image_position(&mut self, id: &str, x: f32, y: f32) {
        if let Some(img) = self.layout.images.iter_mut().find(|i| i.id == id) {
            img.x_mm = x;
            img.y_mm = y;
        }
        self.cache.clear();
    }

    /// Update just the selected image's size without full layout update  
    pub fn update_image_bounds(&mut self, id: &str, x: f32, y: f32, w: f32, h: f32) {
        if let Some(img) = self.layout.images.iter_mut().find(|i| i.id == id) {
            img.x_mm = x;
            img.y_mm = y;
            img.width_mm = w;
            img.height_mm = h;
        }
        self.cache.clear();
    }

    /// Remove an image from source cache when deleted
    pub fn remove_from_source_cache(&mut self, path: &PathBuf) {
        self.source_cache.borrow_mut().remove(path);
    }

    #[allow(dead_code)]
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 5.0);
        self.cache.clear();
    }

    #[allow(dead_code)]
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn mm_to_pixels(&self, mm: f32) -> f32 {
        let pixels_per_mm = 96.0 / 25.4;
        mm * pixels_per_mm * self.zoom
    }

    fn pixels_to_mm(&self, pixels: f32) -> f32 {
        let pixels_per_mm = 96.0 / 25.4;
        pixels / (pixels_per_mm * self.zoom)
    }

    fn draw_content(&self, frame: &mut Frame) {
        let page = &self.layout.page;

        // Draw page background
        let page_width = self.mm_to_pixels(page.width_mm);
        let page_height = self.mm_to_pixels(page.height_mm);

        let page_bg = Path::rectangle(Point::ORIGIN, Size::new(page_width, page_height));
        frame.fill(&page_bg, Color::WHITE);
        frame.stroke(
            &page_bg,
            Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(0.3, 0.3, 0.3)),
        );

        // Draw margins
        let (margin_x, margin_y, printable_width, printable_height) = page.printable_area();
        let margin_rect = Path::rectangle(
            Point::new(self.mm_to_pixels(margin_x), self.mm_to_pixels(margin_y)),
            Size::new(
                self.mm_to_pixels(printable_width),
                self.mm_to_pixels(printable_height),
            ),
        );
        frame.stroke(
            &margin_rect,
            Stroke::default()
                .with_width(1.0)
                .with_color(Color::from_rgb(0.7, 0.7, 0.7)),
        );

        // Get mutable access to caches via RefCell
        let mut image_cache = self.image_cache.borrow_mut();
        let mut source_cache = self.source_cache.borrow_mut();

        // Draw images
        for img in &self.layout.images {
            let x = self.mm_to_pixels(img.x_mm);
            let y = self.mm_to_pixels(img.y_mm);
            let width = self.mm_to_pixels(img.width_mm);
            let height = self.mm_to_pixels(img.height_mm);

            let bounds = Rectangle::new(Point::new(x, y), Size::new(width, height));

            // Try to draw transformed image using Iced 0.13's draw_image
            if let Some(handle) = image_cache.get_transformed_handle(img, &mut source_cache) {
                let image = Image::new(handle);
                frame.draw_image(bounds, image);
            } else {
                // Fallback: draw placeholder rectangle if image can't be loaded
                let image_rect = Path::rectangle(Point::new(x, y), Size::new(width, height));
                frame.fill(&image_rect, Color::from_rgba(0.85, 0.90, 1.0, 0.8));
            }

            // Draw border
            let image_rect = Path::rectangle(Point::new(x, y), Size::new(width, height));
            frame.stroke(
                &image_rect,
                Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgb(0.5, 0.5, 0.5)),
            );

            // Highlight selected image
            if self.layout.selected_image_id.as_ref() == Some(&img.id) {
                frame.stroke(
                    &image_rect,
                    Stroke::default()
                        .with_width(3.0)
                        .with_color(Color::from_rgb(0.0, 0.5, 1.0)),
                );

                // Draw resize handles - corners (larger, square)
                let corner_size = 10.0;
                let corners = [
                    (x, y),                           // TopLeft
                    (x + width, y),                   // TopRight
                    (x, y + height),                  // BottomLeft
                    (x + width, y + height),          // BottomRight
                ];

                for (cx, cy) in corners.iter() {
                    let handle = Path::rectangle(
                        Point::new(cx - corner_size / 2.0, cy - corner_size / 2.0),
                        Size::new(corner_size, corner_size),
                    );
                    frame.fill(&handle, Color::from_rgb(0.0, 0.5, 1.0));
                    frame.stroke(
                        &handle,
                        Stroke::default().with_width(1.0).with_color(Color::WHITE),
                    );
                }

                // Draw edge handles (smaller, centered on edges)
                let edge_size = 8.0;
                let edges = [
                    (x + width / 2.0, y),                  // Top
                    (x + width / 2.0, y + height),         // Bottom
                    (x, y + height / 2.0),                 // Left
                    (x + width, y + height / 2.0),         // Right
                ];

                for (ex, ey) in edges.iter() {
                    let handle = Path::rectangle(
                        Point::new(ex - edge_size / 2.0, ey - edge_size / 2.0),
                        Size::new(edge_size, edge_size),
                    );
                    frame.fill(&handle, Color::from_rgb(0.2, 0.6, 1.0));
                    frame.stroke(
                        &handle,
                        Stroke::default().with_width(1.0).with_color(Color::WHITE),
                    );
                }
            }

            // Draw filename label
            let filename = img
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            let text_bg_width = (filename.len() as f32 * 7.0).max(50.0);
            let text_bg = Path::rectangle(Point::new(x, y), Size::new(text_bg_width, 20.0));
            frame.fill(&text_bg, Color::from_rgba(0.0, 0.0, 0.0, 0.7));

            frame.fill_text(Text {
                content: filename.to_string(),
                position: Point::new(x + 5.0, y + 5.0),
                color: Color::WHITE,
                size: 12.0.into(),
                ..Default::default()
            });
        }
    }

    /// Check if a point (in pixels) is over a resize handle of the selected image
    /// Returns the handle type if found
    fn get_resize_handle_at_point(&self, px: f32, py: f32) -> Option<(String, ResizeHandle)> {
        if let Some(id) = &self.layout.selected_image_id {
            if let Some(img) = self.layout.get_image(id) {
                let x = self.mm_to_pixels(img.x_mm);
                let y = self.mm_to_pixels(img.y_mm);
                let width = self.mm_to_pixels(img.width_mm);
                let height = self.mm_to_pixels(img.height_mm);
                
                let handle_radius = 8.0; // Detection radius
                
                // Check corners first (they have priority)
                let corners = [
                    (x, y, ResizeHandle::TopLeft),
                    (x + width, y, ResizeHandle::TopRight),
                    (x, y + height, ResizeHandle::BottomLeft),
                    (x + width, y + height, ResizeHandle::BottomRight),
                ];
                
                for (cx, cy, handle) in corners.iter() {
                    if (px - cx).abs() < handle_radius && (py - cy).abs() < handle_radius {
                        return Some((id.clone(), *handle));
                    }
                }
                
                // Check edges
                let edges = [
                    (x + width / 2.0, y, ResizeHandle::Top),
                    (x + width / 2.0, y + height, ResizeHandle::Bottom),
                    (x, y + height / 2.0, ResizeHandle::Left),
                    (x + width, y + height / 2.0, ResizeHandle::Right),
                ];
                
                for (ex, ey, handle) in edges.iter() {
                    if (px - ex).abs() < handle_radius && (py - ey).abs() < handle_radius {
                        return Some((id.clone(), *handle));
                    }
                }
            }
        }
        None
    }
}

impl Program<CanvasMessage> for LayoutCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            self.draw_content(frame);
        });

        vec![geometry]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (iced::event::Status, Option<CanvasMessage>) {
        if let Some(cursor_position) = cursor.position_in(bounds) {
            match event {
                canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    // First check if we're clicking on a resize handle
                    if let Some((id, handle)) = self.get_resize_handle_at_point(cursor_position.x, cursor_position.y) {
                        return (
                            iced::event::Status::Captured,
                            Some(CanvasMessage::StartResize(id, handle)),
                        );
                    }
                    
                    // Otherwise check for image selection/move
                    let x_mm = self.pixels_to_mm(cursor_position.x);
                    let y_mm = self.pixels_to_mm(cursor_position.y);

                    if let Some(image) = self.layout.find_image_at_point(x_mm, y_mm) {
                        return (
                            iced::event::Status::Captured,
                            Some(CanvasMessage::SelectImage(image.id.clone())),
                        );
                    } else {
                        return (
                            iced::event::Status::Captured,
                            Some(CanvasMessage::DeselectAll),
                        );
                    }
                }
                canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    let x_mm = self.pixels_to_mm(cursor_position.x);
                    let y_mm = self.pixels_to_mm(cursor_position.y);
                    return (
                        iced::event::Status::Captured,
                        Some(CanvasMessage::MouseMoved(x_mm, y_mm)),
                    );
                }
                canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    return (
                        iced::event::Status::Captured,
                        Some(CanvasMessage::MouseReleased),
                    );
                }
                _ => {}
            }
        }

        (iced::event::Status::Ignored, None)
    }
}
