// layout.rs - Page and image data structures
// Phase 2: Core Layout Engine

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Represents a paper size with physical dimensions in millimeters
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PaperSize {
    // A-series (ISO 216)
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
    // B-series (ISO 216)
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    B10,
    // North American sizes
    Letter,
    Legal,
    Tabloid,
    Ledger,
    // Photo sizes (inches converted to mm)
    Photo3_5x5,      // 3.5" × 5" (88.9 × 127mm)
    Photo4x6,        // 4" × 6" (101.6 × 152.4mm)
    Photo5x5,        // 5" × 5" Square (127 × 127mm)
    Photo5x7,        // 5" × 7" (127 × 177.8mm)
    Photo7x10,       // 7" × 10" (177.8 × 254mm)
    Photo8x10,       // 8" × 10" (203.2 × 254mm)
    Photo10x12,      // 10" × 12" (254 × 304.8mm)
    Photo11x17,      // 11" × 17" Ledger (279.4 × 431.8mm)
    Photo12x12,      // 12" × 12" Square (304.8 × 304.8mm)
    Photo13x19,      // 13" × 19" A3+ (330.2 × 482.6mm)
    Panorama,        // 210 × 594mm
    CustomLarge,     // Custom up to 13" × 39" (330.2 × 990.6mm)
    // Custom size (width, height in mm)
    Custom(f32, f32),
}

#[allow(clippy::wrong_self_convention)]
impl PaperSize {
    /// Convert paper size to dimensions in millimeters (width, height)
    pub fn to_dimensions(&self) -> (f32, f32) {
        match self {
            // A-series: Each size is half the previous one
            PaperSize::A0 => (841.0, 1189.0),
            PaperSize::A1 => (594.0, 841.0),
            PaperSize::A2 => (420.0, 594.0),
            PaperSize::A3 => (297.0, 420.0),
            PaperSize::A4 => (210.0, 297.0),
            PaperSize::A5 => (148.0, 210.0),
            PaperSize::A6 => (105.0, 148.0),
            PaperSize::A7 => (74.0, 105.0),
            PaperSize::A8 => (52.0, 74.0),
            PaperSize::A9 => (37.0, 52.0),
            PaperSize::A10 => (26.0, 37.0),
            // B-series
            PaperSize::B0 => (1000.0, 1414.0),
            PaperSize::B1 => (707.0, 1000.0),
            PaperSize::B2 => (500.0, 707.0),
            PaperSize::B3 => (353.0, 500.0),
            PaperSize::B4 => (250.0, 353.0),
            PaperSize::B5 => (176.0, 250.0),
            PaperSize::B6 => (125.0, 176.0),
            PaperSize::B7 => (88.0, 125.0),
            PaperSize::B8 => (62.0, 88.0),
            PaperSize::B9 => (44.0, 62.0),
            PaperSize::B10 => (31.0, 44.0),
            // North American
            PaperSize::Letter => (215.9, 279.4),  // 8.5" × 11"
            PaperSize::Legal => (215.9, 355.6),   // 8.5" × 14"
            PaperSize::Tabloid => (279.4, 431.8), // 11" × 17"
            PaperSize::Ledger => (431.8, 279.4),  // 17" × 11"
            // Photo sizes
            PaperSize::Photo3_5x5 => (88.9, 127.0),
            PaperSize::Photo4x6 => (101.6, 152.4),
            PaperSize::Photo5x5 => (127.0, 127.0),
            PaperSize::Photo5x7 => (127.0, 177.8),
            PaperSize::Photo7x10 => (177.8, 254.0),
            PaperSize::Photo8x10 => (203.2, 254.0),
            PaperSize::Photo10x12 => (254.0, 304.8),
            PaperSize::Photo11x17 => (279.4, 431.8),
            PaperSize::Photo12x12 => (304.8, 304.8),
            PaperSize::Photo13x19 => (330.2, 482.6),
            PaperSize::Panorama => (210.0, 594.0),
            PaperSize::CustomLarge => (330.2, 990.6), // Max 13" × 39"
            PaperSize::Custom(w, h) => (*w, *h),
        }
    }
}

impl std::fmt::Display for PaperSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaperSize::A0 => write!(f, "A0"),
            PaperSize::A1 => write!(f, "A1"),
            PaperSize::A2 => write!(f, "A2"),
            PaperSize::A3 => write!(f, "A3"),
            PaperSize::A4 => write!(f, "A4"),
            PaperSize::A5 => write!(f, "A5"),
            PaperSize::A6 => write!(f, "A6"),
            PaperSize::A7 => write!(f, "A7"),
            PaperSize::A8 => write!(f, "A8"),
            PaperSize::A9 => write!(f, "A9"),
            PaperSize::A10 => write!(f, "A10"),
            PaperSize::B0 => write!(f, "B0"),
            PaperSize::B1 => write!(f, "B1"),
            PaperSize::B2 => write!(f, "B2"),
            PaperSize::B3 => write!(f, "B3"),
            PaperSize::B4 => write!(f, "B4"),
            PaperSize::B5 => write!(f, "B5"),
            PaperSize::B6 => write!(f, "B6"),
            PaperSize::B7 => write!(f, "B7"),
            PaperSize::B8 => write!(f, "B8"),
            PaperSize::B9 => write!(f, "B9"),
            PaperSize::B10 => write!(f, "B10"),
            PaperSize::Letter => write!(f, "Letter (8.5×11\")"),
            PaperSize::Legal => write!(f, "Legal (8.5×14\")"),
            PaperSize::Tabloid => write!(f, "Tabloid (11×17\")"),
            PaperSize::Ledger => write!(f, "Ledger (17×11\")"),
            PaperSize::Photo3_5x5 => write!(f, "3.5×5\""),
            PaperSize::Photo4x6 => write!(f, "4×6\""),
            PaperSize::Photo5x5 => write!(f, "5×5\" Square"),
            PaperSize::Photo5x7 => write!(f, "5×7\""),
            PaperSize::Photo7x10 => write!(f, "7×10\""),
            PaperSize::Photo8x10 => write!(f, "8×10\""),
            PaperSize::Photo10x12 => write!(f, "10×12\""),
            PaperSize::Photo11x17 => write!(f, "11×17\" Ledger"),
            PaperSize::Photo12x12 => write!(f, "12×12\" Square"),
            PaperSize::Photo13x19 => write!(f, "13×19\" (A3+)"),
            PaperSize::Panorama => write!(f, "210×594mm Panorama"),
            PaperSize::CustomLarge => write!(f, "Custom (up to 13×39\")"),
            PaperSize::Custom(w, h) => write!(f, "Custom ({}×{}mm)", w, h),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for PaperSize {
    fn default() -> Self {
        // Default to A4 (used in most of the world)
        // TODO: Detect locale and return Letter for US/Canada
        PaperSize::A4
    }
}

/// Represents paper type for printing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaperType {
    Plain,              // Plain Paper
    SuperHighGloss,     // Photo Paper Pro Platinum
    Glossy,             // Photo Paper Plus Glossy II, Photo Paper Glossy
    SemiGloss,          // Photo Paper Plus Semi-Gloss, Photo Paper Pro Luster
    Matte,              // Matte Photo Paper, Photo Paper Premium Matte
    FineArt,            // Premium Fine Art Rough
}

impl std::fmt::Display for PaperType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaperType::Plain => write!(f, "Plain Paper"),
            PaperType::SuperHighGloss => write!(f, "Super High Gloss (Pro Platinum)"),
            PaperType::Glossy => write!(f, "Glossy (Plus Glossy II)"),
            PaperType::SemiGloss => write!(f, "Semi-Gloss (Plus Semi-Gloss)"),
            PaperType::Matte => write!(f, "Matte (Premium Matte)"),
            PaperType::FineArt => write!(f, "Fine Art (Premium Rough)"),
        }
    }
}

#[allow(dead_code)]
impl PaperType {
    pub fn as_str(&self) -> &str {
        match self {
            PaperType::Plain => "Plain Paper",
            PaperType::SuperHighGloss => "Super High Gloss",
            PaperType::Glossy => "Glossy",
            PaperType::SemiGloss => "Semi-Gloss",
            PaperType::Matte => "Matte",
            PaperType::FineArt => "Fine Art",
        }
    }
}

impl Default for PaperType {
    fn default() -> Self {
        PaperType::Plain
    }
}

/// Print quality settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PrintQuality {
    Highest,
    High,
    #[default]
    Standard,
    Draft,
}

impl std::fmt::Display for PrintQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintQuality::Highest => write!(f, "Highest"),
            PrintQuality::High => write!(f, "High"),
            PrintQuality::Standard => write!(f, "Standard"),
            PrintQuality::Draft => write!(f, "Draft"),
        }
    }
}

/// Color mode for printing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ColorMode {
    #[default]
    UseICCProfile,
    DriverMatching,
    NoColorCorrection,
    BlackAndWhite,
}

impl std::fmt::Display for ColorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMode::UseICCProfile => write!(f, "Use ICC Profile"),
            ColorMode::DriverMatching => write!(f, "Driver Matching"),
            ColorMode::NoColorCorrection => write!(f, "No Color Correction"),
            ColorMode::BlackAndWhite => write!(f, "Black and White"),
        }
    }
}

/// Page orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Orientation {
    #[default]
    Portrait,
    Landscape,
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Portrait => write!(f, "Portrait"),
            Orientation::Landscape => write!(f, "Landscape"),
        }
    }
}

/// Represents the page configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub width_mm: f32,
    pub height_mm: f32,
    pub margin_top_mm: f32,
    pub margin_bottom_mm: f32,
    pub margin_left_mm: f32,
    pub margin_right_mm: f32,
    pub paper_size: PaperSize,
    pub paper_type: PaperType,
    pub print_quality: PrintQuality,
    pub color_mode: ColorMode,
    pub orientation: Orientation,
    pub borderless: bool,
}

#[allow(dead_code)]
impl Page {
    /// Create a new page with the given paper size and default margins
    pub fn new(paper_size: PaperSize) -> Self {
        let (width_mm, height_mm) = paper_size.to_dimensions();
        Self {
            width_mm,
            height_mm,
            margin_top_mm: 25.4, // 1 inch
            margin_bottom_mm: 25.4,
            margin_left_mm: 25.4,
            margin_right_mm: 25.4,
            paper_size,
            paper_type: PaperType::Plain,
            print_quality: PrintQuality::Standard,
            color_mode: ColorMode::UseICCProfile,
            orientation: Orientation::Portrait,
            borderless: false,
        }
    }

    /// Convert page dimensions to pixels at the given DPI
    pub fn to_pixels(&self, dpi: u32) -> (u32, u32) {
        let width_px = (self.width_mm / 25.4 * dpi as f32) as u32;
        let height_px = (self.height_mm / 25.4 * dpi as f32) as u32;
        (width_px, height_px)
    }

    /// Get the printable area (excluding margins) in millimeters
    pub fn printable_area(&self) -> (f32, f32, f32, f32) {
        let x = self.margin_left_mm;
        let y = self.margin_top_mm;
        let width = self.width_mm - self.margin_left_mm - self.margin_right_mm;
        let height = self.height_mm - self.margin_top_mm - self.margin_bottom_mm;
        (x, y, width, height)
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new(PaperSize::default())
    }
}

/// Represents an image placed on the layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlacedImage {
    pub id: String,
    pub path: PathBuf,
    pub x_mm: f32,
    pub y_mm: f32,
    pub width_mm: f32,
    pub height_mm: f32,
    pub rotation_degrees: f32,
    pub z_index: usize,
    pub original_width_px: u32,
    pub original_height_px: u32,
    pub locked: bool,
    /// Horizontal flip (mirror)
    #[serde(default)]
    pub flip_horizontal: bool,
    /// Vertical flip
    #[serde(default)]
    pub flip_vertical: bool,
    /// Opacity (0.0 = transparent, 1.0 = fully opaque)
    #[serde(default = "default_opacity")]
    pub opacity: f32,
}

fn default_opacity() -> f32 {
    1.0
}

#[allow(dead_code)]
impl PlacedImage {
    /// Create a new placed image with default positioning
    pub fn new(path: PathBuf, original_width_px: u32, original_height_px: u32) -> Self {
        let id = Uuid::new_v4().to_string();
        // Default size: 100mm width, maintaining aspect ratio
        let aspect_ratio = original_height_px as f32 / original_width_px as f32;
        let width_mm = 100.0;
        let height_mm = width_mm * aspect_ratio;

        Self {
            id,
            path,
            x_mm: 50.0,
            y_mm: 50.0,
            width_mm,
            height_mm,
            rotation_degrees: 0.0,
            z_index: 0,
            original_width_px,
            original_height_px,
            locked: false,
            flip_horizontal: false,
            flip_vertical: false,
            opacity: 1.0,
        }
    }

    /// Calculate the effective DPI when this image is printed
    pub fn effective_dpi(&self) -> (f32, f32) {
        let width_inches = self.width_mm / 25.4;
        let height_inches = self.height_mm / 25.4;
        let dpi_x = self.original_width_px as f32 / width_inches;
        let dpi_y = self.original_height_px as f32 / height_inches;
        (dpi_x, dpi_y)
    }

    /// Check if a point (in mm) is within this image's bounds
    pub fn contains_point(&self, x_mm: f32, y_mm: f32) -> bool {
        x_mm >= self.x_mm
            && x_mm <= self.x_mm + self.width_mm
            && y_mm >= self.y_mm
            && y_mm <= self.y_mm + self.height_mm
    }

    /// Get the bounding box in millimeters (x, y, width, height)
    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        (self.x_mm, self.y_mm, self.width_mm, self.height_mm)
    }
}

/// Represents the complete layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    pub page: Page,
    pub images: Vec<PlacedImage>,
    pub selected_image_id: Option<String>,
}

#[allow(dead_code)]
impl Layout {
    /// Create a new empty layout with default page settings
    pub fn new() -> Self {
        Self {
            page: Page::default(),
            images: Vec::new(),
            selected_image_id: None,
        }
    }

    /// Add an image to the layout
    pub fn add_image(&mut self, image: PlacedImage) {
        let z_index = self.images.len();
        let mut image = image;
        image.z_index = z_index;
        self.images.push(image);
    }

    /// Remove an image by ID
    pub fn remove_image(&mut self, id: &str) -> Option<PlacedImage> {
        if let Some(index) = self.images.iter().position(|img| img.id == id) {
            let removed = self.images.remove(index);
            // Reindex remaining images
            for (i, img) in self.images.iter_mut().enumerate() {
                img.z_index = i;
            }
            // Clear selection if removed image was selected
            if self.selected_image_id.as_deref() == Some(id) {
                self.selected_image_id = None;
            }
            Some(removed)
        } else {
            None
        }
    }

    /// Get a mutable reference to an image by ID
    pub fn get_image_mut(&mut self, id: &str) -> Option<&mut PlacedImage> {
        self.images.iter_mut().find(|img| img.id == id)
    }

    /// Get an immutable reference to an image by ID
    pub fn get_image(&self, id: &str) -> Option<&PlacedImage> {
        self.images.iter().find(|img| img.id == id)
    }

    /// Find the topmost image at the given point (in mm)
    pub fn find_image_at_point(&self, x_mm: f32, y_mm: f32) -> Option<&PlacedImage> {
        // Iterate in reverse z-order (topmost first)
        self.images
            .iter()
            .rev()
            .find(|img| img.contains_point(x_mm, y_mm))
    }

    /// Get the currently selected image
    pub fn selected_image(&self) -> Option<&PlacedImage> {
        self.selected_image_id
            .as_ref()
            .and_then(|id| self.get_image(id))
    }

    /// Get a mutable reference to the currently selected image
    pub fn selected_image_mut(&mut self) -> Option<&mut PlacedImage> {
        let id = self.selected_image_id.clone()?;
        self.get_image_mut(&id)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}
