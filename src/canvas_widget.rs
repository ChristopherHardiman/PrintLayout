// canvas_widget.rs - Canvas widget implementation with image rendering
// Updated for Iced 0.13 with draw_image support

use crate::layout::Layout;
use iced::mouse::{self, Cursor};
use iced::widget::canvas::{self, Cache, Frame, Geometry, Image, Path, Program, Stroke, Text};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

/// Image handle cache to avoid recreating handles
#[derive(Debug, Default)]
pub struct ImageCache {
    cache: HashMap<PathBuf, iced::widget::image::Handle>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get or create an image handle for the given path
    pub fn get_handle(&mut self, path: &PathBuf) -> Option<iced::widget::image::Handle> {
        if let Some(handle) = self.cache.get(path) {
            return Some(handle.clone());
        }

        if path.exists() {
            let handle = iced::widget::image::Handle::from_path(path);
            self.cache.insert(path.clone(), handle.clone());
            Some(handle)
        } else {
            None
        }
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
    // Use RefCell for interior mutability to allow caching in draw()
    image_cache: RefCell<ImageCache>,
}

impl LayoutCanvas {
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            zoom: 1.0,
            cache: Cache::new(),
            image_cache: RefCell::new(ImageCache::new()),
        }
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.cache.clear();
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

        // Get mutable access to image cache via RefCell
        let mut image_cache = self.image_cache.borrow_mut();

        // Draw images
        for img in &self.layout.images {
            let x = self.mm_to_pixels(img.x_mm);
            let y = self.mm_to_pixels(img.y_mm);
            let width = self.mm_to_pixels(img.width_mm);
            let height = self.mm_to_pixels(img.height_mm);

            let bounds = Rectangle::new(Point::new(x, y), Size::new(width, height));

            // Try to draw actual image using Iced 0.13's draw_image
            if let Some(handle) = image_cache.get_handle(&img.path) {
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

                // Draw resize handles
                let handle_size = 8.0;
                let corners = [
                    (x, y),
                    (x + width, y),
                    (x, y + height),
                    (x + width, y + height),
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
