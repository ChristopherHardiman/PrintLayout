// canvas_widget.rs - Canvas widget implementation
// Phase 2: Custom Canvas Widget and Image Cache

use crate::layout::Layout;
use iced::mouse::{self, Cursor};
use iced::widget::canvas::{self, Cache, Frame, Geometry, Path, Program, Stroke, Text};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use std::collections::HashMap;
use std::path::PathBuf;

/// Image cache to avoid reloading images
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ImageCache {
    cache: HashMap<PathBuf, image::DynamicImage>,
    max_size_mb: usize,
    current_size_mb: usize,
}

#[allow(dead_code)]
impl ImageCache {
    /// Create a new image cache with default size limit (500MB)
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_size_mb: 500,
            current_size_mb: 0,
        }
    }

    /// Load an image from cache or from disk
    pub fn load(&mut self, path: &PathBuf) -> Option<image::DynamicImage> {
        if let Some(img) = self.cache.get(path) {
            Some(img.clone())
        } else {
            // Load image from disk
            if let Ok(img) = image::open(path) {
                self.cache.insert(path.clone(), img.clone());
                Some(img)
            } else {
                None
            }
        }
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.current_size_mb = 0;
    }

    /// Remove a specific image from cache
    pub fn invalidate(&mut self, path: &PathBuf) {
        self.cache.remove(path);
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
}

/// The canvas widget for displaying and interacting with the layout
pub struct LayoutCanvas {
    layout: Layout,
    zoom: f32,
    cache: Cache,
    #[allow(dead_code)]
    image_cache: ImageCache,
}
#[allow(dead_code)]
impl LayoutCanvas {
    /// Create a new layout canvas
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            zoom: 1.0,
            cache: Cache::new(),
            image_cache: ImageCache::new(),
        }
    }

    /// Update the layout
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.cache.clear();
    }

    /// Get the current layout
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    /// Set the zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 5.0);
        self.cache.clear();
    }

    /// Get the current zoom level
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Convert millimeters to pixels for rendering
    fn mm_to_pixels(&self, mm: f32) -> f32 {
        // Assume 96 DPI for screen rendering
        let pixels_per_mm = 96.0 / 25.4;
        mm * pixels_per_mm * self.zoom
    }

    /// Convert pixels to millimeters
    fn pixels_to_mm(&self, pixels: f32) -> f32 {
        let pixels_per_mm = 96.0 / 25.4;
        pixels / (pixels_per_mm * self.zoom)
    }

    /// Draw the canvas content
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

        // Draw images
        for image in &self.layout.images {
            let x = self.mm_to_pixels(image.x_mm);
            let y = self.mm_to_pixels(image.y_mm);
            let width = self.mm_to_pixels(image.width_mm);
            let height = self.mm_to_pixels(image.height_mm);

            // Draw image placeholder (colored rectangle)
            let image_rect = Path::rectangle(Point::new(x, y), Size::new(width, height));
            frame.fill(&image_rect, Color::from_rgb(0.9, 0.9, 1.0));
            frame.stroke(
                &image_rect,
                Stroke::default()
                    .with_width(1.0)
                    .with_color(Color::from_rgb(0.5, 0.5, 0.5)),
            );

            // Highlight selected image
            if self.layout.selected_image_id.as_ref() == Some(&image.id) {
                frame.stroke(
                    &image_rect,
                    Stroke::default()
                        .with_width(3.0)
                        .with_color(Color::from_rgb(0.0, 0.5, 1.0)),
                );

                // Draw resize handles at corners
                let handle_size = 8.0;
                let corners = [
                    (x, y),                  // Top-left
                    (x + width, y),          // Top-right
                    (x, y + height),         // Bottom-left
                    (x + width, y + height), // Bottom-right
                ];

                for (cx, cy) in corners.iter() {
                    let handle = Path::rectangle(
                        Point::new(cx - handle_size / 2.0, cy - handle_size / 2.0),
                        Size::new(handle_size, handle_size),
                    );
                    frame.fill(&handle, Color::from_rgb(0.0, 0.5, 1.0));
                    frame.stroke(
                        &handle,
                        Stroke::default().with_width(1.0).with_color(Color::WHITE),
                    );
                }
            }

            // Draw image filename
            let filename = image
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            frame.fill_text(Text {
                content: filename.to_string(),
                position: Point::new(x + 5.0, y + 5.0),
                color: Color::BLACK,
                size: 12.0.into(),
                ..Default::default()
            });
        }
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
                    // Convert screen coordinates to layout coordinates
                    let x_mm = self.pixels_to_mm(cursor_position.x);
                    let y_mm = self.pixels_to_mm(cursor_position.y);

                    // Check if clicking on an image
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
                    // Emit cursor position for drag handling
                    let x_mm = self.pixels_to_mm(cursor_position.x);
                    let y_mm = self.pixels_to_mm(cursor_position.y);
                    return (
                        iced::event::Status::Captured,
                        Some(CanvasMessage::MouseMoved(x_mm, y_mm)),
                    );
                }
                canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    // Stop dragging
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
