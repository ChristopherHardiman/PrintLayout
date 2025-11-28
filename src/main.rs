use iced::widget::{
    button, canvas, column, container, pick_list, row, scrollable, text, text_input,
    horizontal_rule, vertical_rule, checkbox, Space, image as iced_image, center,
    progress_bar, opaque, mouse_area,
};
use iced::{Alignment, Color, Element, Length, Padding, Size, Task, Theme};
use ::image::GenericImageView;
use std::collections::HashMap;
use std::path::PathBuf;

mod canvas_widget;
mod config;
mod layout;
mod printing;

use canvas_widget::{CanvasMessage, LayoutCanvas, ResizeHandle};
use config::{ConfigManager, ProjectLayout, UserPreferences};
use layout::{Layout, PaperSize, PaperType, PlacedImage, PrintQuality, ColorMode, Orientation as LayoutOrientation};
use printing::{discover_printers, execute_print_job, PrintJob, PrinterInfo};

pub fn main() -> iced::Result {
    env_logger::init();
    log::info!("Initializing Print Layout v{}", VERSION);
    
    iced::application(PrintLayout::title, PrintLayout::update, PrintLayout::view)
        .theme(PrintLayout::theme)
        .window_size(Size::new(1400.0, 900.0))
        .run_with(PrintLayout::new)
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings panel tabs (mimicking Canon PPL)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    PrintSettings,
    Layout,
    ColorManagement,
    ImageTools,
}

/// Print job status for progress dialog
#[derive(Debug, Clone, PartialEq)]
pub enum PrintStatus {
    Idle,
    Rendering,
    Sending,
    Completed(String),  // Job ID
    Failed(String),     // Error message
}

#[derive(Debug, Clone)]
pub enum Message {
    CanvasMessage(CanvasMessage),
    AddImageClicked,
    ImageFilesSelected(Vec<PathBuf>),
    DeleteImageClicked,
    PaperSizeSelected(PaperSize),
    PaperTypeSelected(PaperType),
    MarginTopChanged(String),
    MarginBottomChanged(String),
    MarginLeftChanged(String),
    MarginRightChanged(String),
    ZoomIn,
    ZoomOut,
    ZoomReset,
    ZoomToFit,
    // New settings messages
    SettingsTabChanged(SettingsTab),
    PrintQualitySelected(PrintQuality),
    ColorModeSelected(ColorMode),
    OrientationToggled,
    BorderlessToggled(bool),
    CopiesChanged(String),
    // Thumbnail operations
    ThumbnailClicked(String),
    ImageCopiesChanged(String, String),
    // Image manipulation tools
    RotateImageCW,           // Rotate 90° clockwise
    RotateImageCCW,          // Rotate 90° counter-clockwise
    FlipImageHorizontal,     // Mirror horizontally
    FlipImageVertical,       // Flip vertically
    ImageOpacityChanged(String),  // Change opacity (0-100%)
    ImageWidthChanged(String),    // Resize width in mm
    ImageHeightChanged(String),   // Resize height in mm
    MaintainAspectRatio(bool),    // Toggle aspect ratio lock
    // Printing messages
    PrintersDiscovered(Vec<PrinterInfo>),
    PrinterSelected(String),
    PrintClicked,
    PrintJobCompleted(Result<String, String>),
    DismissPrintStatus,
    // File operations
    NewLayout,
    SaveLayoutClicked,
    SaveLayoutAs,
    LayoutSavePathSelected(Option<PathBuf>),
    OpenLayoutClicked,
    LayoutOpenPathSelected(Option<PathBuf>),
    LayoutLoaded(Result<ProjectLayout, String>),
    CheckAutoSave,
    RecoverAutoSave,
    DiscardAutoSave,
    AutoSaveTick,
    // Recent files
    OpenRecentFile(PathBuf),
    ToggleRecentFilesMenu,
}

/// Tracks what kind of drag operation is in progress
#[derive(Debug, Clone, Copy, PartialEq)]
enum DragMode {
    None,
    Move,
    Resize(ResizeHandle),
}

struct PrintLayout {
    layout: Layout,
    canvas: LayoutCanvas,
    zoom: f32,
    margin_top_input: String,
    margin_bottom_input: String,
    margin_left_input: String,
    margin_right_input: String,
    // Drag state
    drag_mode: DragMode,
    drag_start_pos: (f32, f32),
    drag_image_initial_pos: (f32, f32),
    drag_image_initial_size: (f32, f32),
    // Printing state
    printers: Vec<PrinterInfo>,
    selected_printer: Option<String>,
    print_copies: u32,
    print_dpi: u32,
    copies_input: String,
    // UI state
    settings_tab: SettingsTab,
    print_status: PrintStatus,
    // Image manipulation state
    image_width_input: String,
    image_height_input: String,
    image_opacity_input: String,
    maintain_aspect_ratio: bool,
    // Config and file state
    config_manager: ConfigManager,
    preferences: UserPreferences,
    current_file: Option<PathBuf>,
    project: Option<ProjectLayout>,
    is_modified: bool,
    auto_save_counter: u32,
    // UI dialogs/menus state
    show_recent_files_menu: bool,
    show_recovery_dialog: bool,
    // Thumbnail cache for performance
    thumbnail_cache: HashMap<PathBuf, iced::widget::image::Handle>,
    // Cached string for zoom percentage display
    zoom_text: String,
}

impl PrintLayout {
    fn new() -> (Self, Task<Message>) {
        // Initialize config manager
        let config_manager = ConfigManager::new().expect("Failed to create config manager");
        let preferences = config_manager.load_config();
        
        // Create layout with preferences, applying last successful print settings if available
        let mut layout = Layout::new();
        
        // Apply last print settings if they exist
        let last_print = &preferences.last_print_settings;
        if let Some(paper_size) = last_print.paper_size {
            layout.page.paper_size = paper_size;
        }
        if let Some(paper_type) = last_print.paper_type {
            layout.page.paper_type = paper_type;
        }
        if let Some(print_quality) = last_print.print_quality {
            layout.page.print_quality = print_quality;
        }
        if let Some(color_mode) = last_print.color_mode {
            layout.page.color_mode = color_mode;
        }
        if let Some(orientation) = last_print.orientation {
            layout.page.orientation = orientation;
        }
        if let Some(borderless) = last_print.borderless {
            layout.page.borderless = borderless;
        }
        if let Some(margins) = last_print.margins {
            layout.page.margin_top_mm = margins.0;
            layout.page.margin_bottom_mm = margins.1;
            layout.page.margin_left_mm = margins.2;
            layout.page.margin_right_mm = margins.3;
        }
        
        let canvas = LayoutCanvas::new(layout.clone());
        
        // Use margins from last print settings if available, otherwise use defaults
        let (margin_top, margin_bottom, margin_left, margin_right) = 
            last_print.margins.unwrap_or(preferences.default_margins);
        
        // Get copies from last print, default to 1
        let print_copies = last_print.copies.unwrap_or(1);
        
        // Pre-compute zoom text for display
        let zoom_text = format!("{:.0}%", preferences.zoom_level * 100.0);

        let instance = PrintLayout {
            layout,
            canvas,
            zoom: preferences.zoom_level,
            margin_top_input: margin_top.to_string(),
            margin_bottom_input: margin_bottom.to_string(),
            margin_left_input: margin_left.to_string(),
            margin_right_input: margin_right.to_string(),
            drag_mode: DragMode::None,
            drag_start_pos: (0.0, 0.0),
            drag_image_initial_pos: (0.0, 0.0),
            drag_image_initial_size: (0.0, 0.0),
            printers: Vec::new(),
            // Use printer from last print settings if available
            selected_printer: last_print.printer_name.clone().or(preferences.last_printer.clone()),
            print_copies,
            print_dpi: 300,
            copies_input: print_copies.to_string(),
            settings_tab: SettingsTab::PrintSettings,
            print_status: PrintStatus::Idle,
            // Image manipulation defaults
            image_width_input: String::new(),
            image_height_input: String::new(),
            image_opacity_input: "100".to_string(),
            maintain_aspect_ratio: true,
            config_manager,
            preferences,
            current_file: None,
            project: None,
            is_modified: false,
            auto_save_counter: 0,
            show_recent_files_menu: false,
            show_recovery_dialog: false,
            thumbnail_cache: HashMap::new(),
            zoom_text,
        };
        
        let mut tasks = vec![
            Task::perform(
                async {
                    discover_printers().unwrap_or_else(|e| {
                        log::error!("Failed to discover printers: {}", e);
                        Vec::new()
                    })
                },
                Message::PrintersDiscovered,
            ),
            Task::done(Message::CheckAutoSave),
        ];
        
        // Set up auto-save timer if enabled
        if instance.preferences.auto_save_enabled {
            tasks.push(Task::done(Message::AutoSaveTick));
        }
        
        (instance, Task::batch(tasks))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CanvasMessage(canvas_msg) => match canvas_msg {
                CanvasMessage::SelectImage(id) => {
                    log::info!("Selected image: {}", id);
                    self.layout.selected_image_id = Some(id.clone());
                    if let Some(image) = self.layout.get_image(&id) {
                        self.drag_mode = DragMode::Move;
                        self.drag_image_initial_pos = (image.x_mm, image.y_mm);
                        self.drag_image_initial_size = (image.width_mm, image.height_mm);
                        self.drag_start_pos = (0.0, 0.0);
                        // Update input fields for the selected image
                        self.image_width_input = format!("{:.1}", image.width_mm);
                        self.image_height_input = format!("{:.1}", image.height_mm);
                        self.image_opacity_input = format!("{:.0}", image.opacity * 100.0);
                    }
                    self.canvas.set_layout(self.layout.clone());
                }
                CanvasMessage::StartResize(id, handle) => {
                    log::info!("Start resize: {} with handle {:?}", id, handle);
                    self.layout.selected_image_id = Some(id.clone());
                    if let Some(image) = self.layout.get_image(&id) {
                        self.drag_mode = DragMode::Resize(handle);
                        self.drag_image_initial_pos = (image.x_mm, image.y_mm);
                        self.drag_image_initial_size = (image.width_mm, image.height_mm);
                        self.drag_start_pos = (0.0, 0.0);
                    }
                    self.canvas.set_layout(self.layout.clone());
                }
                CanvasMessage::DeselectAll => {
                    self.layout.selected_image_id = None;
                    self.drag_mode = DragMode::None;
                    self.canvas.set_layout(self.layout.clone());
                }
                CanvasMessage::MouseMoved(x, y) => {
                    match self.drag_mode {
                        DragMode::Move => {
                            if let Some(id) = self.layout.selected_image_id.clone() {
                                if self.drag_start_pos == (0.0, 0.0) {
                                    self.drag_start_pos = (x, y);
                                }
                                let dx = x - self.drag_start_pos.0;
                                let dy = y - self.drag_start_pos.1;
                                let new_x = self.drag_image_initial_pos.0 + dx;
                                let new_y = self.drag_image_initial_pos.1 + dy;
                                // Update layout directly
                                if let Some(image) = self.layout.get_image_mut(&id) {
                                    image.x_mm = new_x;
                                    image.y_mm = new_y;
                                }
                                // Use optimized method that updates canvas position directly
                                self.canvas.update_image_position(&id, new_x, new_y);
                            }
                        }
                        DragMode::Resize(handle) => {
                            if let Some(id) = self.layout.selected_image_id.clone() {
                                if self.drag_start_pos == (0.0, 0.0) {
                                    self.drag_start_pos = (x, y);
                                }
                                let dx = x - self.drag_start_pos.0;
                                let dy = y - self.drag_start_pos.1;
                                
                                let (init_x, init_y) = self.drag_image_initial_pos;
                                let (init_w, init_h) = self.drag_image_initial_size;
                                let aspect_ratio = init_w / init_h;
                                
                                let (new_x, new_y, new_w, new_h) = match handle {
                                    ResizeHandle::BottomRight => {
                                        let new_w = (init_w + dx).max(10.0);
                                        let new_h = if self.maintain_aspect_ratio {
                                            new_w / aspect_ratio
                                        } else {
                                            (init_h + dy).max(10.0)
                                        };
                                        (init_x, init_y, new_w, new_h)
                                    }
                                    ResizeHandle::BottomLeft => {
                                        let new_w = (init_w - dx).max(10.0);
                                        let new_h = if self.maintain_aspect_ratio {
                                            new_w / aspect_ratio
                                        } else {
                                            (init_h + dy).max(10.0)
                                        };
                                        let new_x = init_x + init_w - new_w;
                                        (new_x, init_y, new_w, new_h)
                                    }
                                    ResizeHandle::TopRight => {
                                        let new_w = (init_w + dx).max(10.0);
                                        let new_h = if self.maintain_aspect_ratio {
                                            new_w / aspect_ratio
                                        } else {
                                            (init_h - dy).max(10.0)
                                        };
                                        let new_y = init_y + init_h - new_h;
                                        (init_x, new_y, new_w, new_h)
                                    }
                                    ResizeHandle::TopLeft => {
                                        let new_w = (init_w - dx).max(10.0);
                                        let new_h = if self.maintain_aspect_ratio {
                                            new_w / aspect_ratio
                                        } else {
                                            (init_h - dy).max(10.0)
                                        };
                                        let new_x = init_x + init_w - new_w;
                                        let new_y = init_y + init_h - new_h;
                                        (new_x, new_y, new_w, new_h)
                                    }
                                    ResizeHandle::Right => {
                                        let new_w = (init_w + dx).max(10.0);
                                        let new_h = if self.maintain_aspect_ratio {
                                            new_w / aspect_ratio
                                        } else {
                                            init_h
                                        };
                                        (init_x, init_y, new_w, new_h)
                                    }
                                    ResizeHandle::Left => {
                                        let new_w = (init_w - dx).max(10.0);
                                        let new_h = if self.maintain_aspect_ratio {
                                            new_w / aspect_ratio
                                        } else {
                                            init_h
                                        };
                                        let new_x = init_x + init_w - new_w;
                                        (new_x, init_y, new_w, new_h)
                                    }
                                    ResizeHandle::Bottom => {
                                        let new_h = (init_h + dy).max(10.0);
                                        let new_w = if self.maintain_aspect_ratio {
                                            new_h * aspect_ratio
                                        } else {
                                            init_w
                                        };
                                        (init_x, init_y, new_w, new_h)
                                    }
                                    ResizeHandle::Top => {
                                        let new_h = (init_h - dy).max(10.0);
                                        let new_w = if self.maintain_aspect_ratio {
                                            new_h * aspect_ratio
                                        } else {
                                            init_w
                                        };
                                        let new_y = init_y + init_h - new_h;
                                        (init_x, new_y, new_w, new_h)
                                    }
                                };
                                
                                if let Some(image) = self.layout.get_image_mut(&id) {
                                    image.x_mm = new_x;
                                    image.y_mm = new_y;
                                    image.width_mm = new_w;
                                    image.height_mm = new_h;
                                    // Update input fields live
                                    self.image_width_input = format!("{:.1}", new_w);
                                    self.image_height_input = format!("{:.1}", new_h);
                                }
                                // Use optimized method that updates canvas bounds directly
                                self.canvas.update_image_bounds(&id, new_x, new_y, new_w, new_h);
                            }
                        }
                        DragMode::None => {}
                    }
                }
                CanvasMessage::MouseReleased => {
                    if self.drag_mode != DragMode::None {
                        self.drag_mode = DragMode::None;
                        self.drag_start_pos = (0.0, 0.0);
                        self.is_modified = true;
                    }
                }
                CanvasMessage::ImageMoved(id, x, y) => {
                    if let Some(image) = self.layout.get_image_mut(&id) {
                        image.x_mm = x;
                        image.y_mm = y;
                        self.canvas.set_layout(self.layout.clone());
                    }
                }
                CanvasMessage::ImageResized(id, width, height) => {
                    if let Some(image) = self.layout.get_image_mut(&id) {
                        image.width_mm = width;
                        image.height_mm = height;
                        self.canvas.set_layout(self.layout.clone());
                    }
                }
                CanvasMessage::CanvasClicked(_, _) => {}
            },
            Message::AddImageClicked => {
                return Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp", "webp"])
                            .set_title("Select Images to Add")
                            .pick_files()
                            .await
                            .map(|files| files.into_iter().map(|f| f.path().to_path_buf()).collect())
                            .unwrap_or_default()
                    },
                    Message::ImageFilesSelected,
                );
            }
            Message::ImageFilesSelected(paths) => {
                for path in paths {
                    match ::image::open(&path) {
                        Ok(img) => {
                            let (width, height) = img.dimensions();
                            let placed_image = PlacedImage::new(path.clone(), width, height);
                            self.layout.add_image(placed_image);
                            // Cache the thumbnail handle
                            let handle = iced::widget::image::Handle::from_path(&path);
                            self.thumbnail_cache.insert(path.clone(), handle);
                            log::info!("Added image: {} ({}x{})", path.display(), width, height);
                        }
                        Err(e) => log::error!("Failed to load image {}: {}", path.display(), e),
                    }
                }
                self.canvas.set_layout(self.layout.clone());
                self.is_modified = true;
            }
            Message::DeleteImageClicked => {
                if let Some(id) = &self.layout.selected_image_id.clone() {
                    // Remove from thumbnail cache and source cache
                    if let Some(img) = self.layout.get_image(id) {
                        self.thumbnail_cache.remove(&img.path);
                        self.canvas.remove_from_source_cache(&img.path);
                    }
                    self.layout.remove_image(id);
                    self.canvas.set_layout(self.layout.clone());
                    self.is_modified = true;
                }
            }
            Message::PaperSizeSelected(paper_size) => {
                let (width, height) = paper_size.to_dimensions();
                self.layout.page.width_mm = width;
                self.layout.page.height_mm = height;
                self.layout.page.paper_size = paper_size;
                self.canvas.set_layout(self.layout.clone());
                self.is_modified = true;
            }
            Message::PaperTypeSelected(paper_type) => {
                self.layout.page.paper_type = paper_type;
                self.is_modified = true;
            }
            Message::MarginTopChanged(value) => {
                self.margin_top_input = value.clone();
                if let Ok(margin) = value.parse::<f32>() {
                    if margin >= 0.0 && margin < self.layout.page.height_mm / 2.0 {
                        self.layout.page.margin_top_mm = margin;
                        self.canvas.set_layout(self.layout.clone());
                    }
                }
            }
            Message::MarginBottomChanged(value) => {
                self.margin_bottom_input = value.clone();
                if let Ok(margin) = value.parse::<f32>() {
                    if margin >= 0.0 && margin < self.layout.page.height_mm / 2.0 {
                        self.layout.page.margin_bottom_mm = margin;
                        self.canvas.set_layout(self.layout.clone());
                    }
                }
            }
            Message::MarginLeftChanged(value) => {
                self.margin_left_input = value.clone();
                if let Ok(margin) = value.parse::<f32>() {
                    if margin >= 0.0 && margin < self.layout.page.width_mm / 2.0 {
                        self.layout.page.margin_left_mm = margin;
                        self.canvas.set_layout(self.layout.clone());
                    }
                }
            }
            Message::MarginRightChanged(value) => {
                self.margin_right_input = value.clone();
                if let Ok(margin) = value.parse::<f32>() {
                    if margin >= 0.0 && margin < self.layout.page.width_mm / 2.0 {
                        self.layout.page.margin_right_mm = margin;
                        self.canvas.set_layout(self.layout.clone());
                    }
                }
            }
            Message::ZoomIn => {
                self.zoom = (self.zoom * 1.2).min(5.0);
                self.zoom_text = format!("{:.0}%", self.zoom * 100.0);
                self.canvas.set_zoom(self.zoom);
            }
            Message::ZoomOut => {
                self.zoom = (self.zoom / 1.2).max(0.1);
                self.zoom_text = format!("{:.0}%", self.zoom * 100.0);
                self.canvas.set_zoom(self.zoom);
            }
            Message::ZoomReset => {
                self.zoom = 1.0;
                self.zoom_text = "100%".to_string();
                self.canvas.set_zoom(self.zoom);
            }
            Message::ZoomToFit => {
                // Fit the page to the canvas (simplified implementation)
                self.zoom = 0.5;
                self.zoom_text = "50%".to_string();
                self.canvas.set_zoom(self.zoom);
            }
            // New settings handlers
            Message::SettingsTabChanged(tab) => {
                self.settings_tab = tab;
            }
            Message::PrintQualitySelected(quality) => {
                self.layout.page.print_quality = quality;
                self.is_modified = true;
            }
            Message::ColorModeSelected(mode) => {
                self.layout.page.color_mode = mode;
                self.is_modified = true;
            }
            Message::OrientationToggled => {
                // Swap dimensions and toggle orientation
                let new_orientation = match self.layout.page.orientation {
                    LayoutOrientation::Portrait => LayoutOrientation::Landscape,
                    LayoutOrientation::Landscape => LayoutOrientation::Portrait,
                };
                std::mem::swap(&mut self.layout.page.width_mm, &mut self.layout.page.height_mm);
                self.layout.page.orientation = new_orientation;
                self.canvas.set_layout(self.layout.clone());
                self.is_modified = true;
            }
            Message::BorderlessToggled(enabled) => {
                self.layout.page.borderless = enabled;
                if enabled {
                    self.layout.page.margin_top_mm = 0.0;
                    self.layout.page.margin_bottom_mm = 0.0;
                    self.layout.page.margin_left_mm = 0.0;
                    self.layout.page.margin_right_mm = 0.0;
                    self.margin_top_input = "0".to_string();
                    self.margin_bottom_input = "0".to_string();
                    self.margin_left_input = "0".to_string();
                    self.margin_right_input = "0".to_string();
                } else {
                    self.layout.page.margin_top_mm = 25.4;
                    self.layout.page.margin_bottom_mm = 25.4;
                    self.layout.page.margin_left_mm = 25.4;
                    self.layout.page.margin_right_mm = 25.4;
                    self.margin_top_input = "25.4".to_string();
                    self.margin_bottom_input = "25.4".to_string();
                    self.margin_left_input = "25.4".to_string();
                    self.margin_right_input = "25.4".to_string();
                }
                self.canvas.set_layout(self.layout.clone());
                self.is_modified = true;
            }
            Message::CopiesChanged(value) => {
                self.copies_input = value.clone();
                if let Ok(copies) = value.parse::<u32>() {
                    if copies >= 1 && copies <= 99 {
                        self.print_copies = copies;
                    }
                }
            }
            Message::ThumbnailClicked(id) => {
                self.layout.selected_image_id = Some(id.clone());
                // Update the image input fields to reflect selected image
                if let Some(img) = self.layout.get_image(&id) {
                    self.image_width_input = format!("{:.1}", img.width_mm);
                    self.image_height_input = format!("{:.1}", img.height_mm);
                    self.image_opacity_input = format!("{:.0}", img.opacity * 100.0);
                }
                self.canvas.set_layout(self.layout.clone());
            }
            Message::ImageCopiesChanged(_id, _value) => {
                // Per-image copies (future implementation)
            }
            // Image manipulation tools
            Message::RotateImageCW => {
                if let Some(img) = self.layout.selected_image_mut() {
                    // Rotate 90° clockwise - swap width and height
                    std::mem::swap(&mut img.width_mm, &mut img.height_mm);
                    img.rotation_degrees = (img.rotation_degrees + 90.0) % 360.0;
                    // Update input fields
                    self.image_width_input = format!("{:.1}", img.width_mm);
                    self.image_height_input = format!("{:.1}", img.height_mm);
                    self.canvas.set_layout(self.layout.clone());
                    self.is_modified = true;
                }
            }
            Message::RotateImageCCW => {
                if let Some(img) = self.layout.selected_image_mut() {
                    // Rotate 90° counter-clockwise - swap width and height
                    std::mem::swap(&mut img.width_mm, &mut img.height_mm);
                    img.rotation_degrees = (img.rotation_degrees + 270.0) % 360.0;
                    // Update input fields
                    self.image_width_input = format!("{:.1}", img.width_mm);
                    self.image_height_input = format!("{:.1}", img.height_mm);
                    self.canvas.set_layout(self.layout.clone());
                    self.is_modified = true;
                }
            }
            Message::FlipImageHorizontal => {
                if let Some(img) = self.layout.selected_image_mut() {
                    img.flip_horizontal = !img.flip_horizontal;
                    self.canvas.set_layout(self.layout.clone());
                    self.is_modified = true;
                }
            }
            Message::FlipImageVertical => {
                if let Some(img) = self.layout.selected_image_mut() {
                    img.flip_vertical = !img.flip_vertical;
                    self.canvas.set_layout(self.layout.clone());
                    self.is_modified = true;
                }
            }
            Message::ImageOpacityChanged(value) => {
                self.image_opacity_input = value.clone();
                if let Ok(opacity) = value.parse::<f32>() {
                    let clamped = (opacity / 100.0).clamp(0.0, 1.0);
                    if let Some(img) = self.layout.selected_image_mut() {
                        img.opacity = clamped;
                        self.canvas.set_layout(self.layout.clone());
                        self.is_modified = true;
                    }
                }
            }
            Message::ImageWidthChanged(value) => {
                self.image_width_input = value.clone();
                if let Ok(new_width) = value.parse::<f32>() {
                    if new_width > 0.0 {
                        if let Some(img) = self.layout.selected_image_mut() {
                            if self.maintain_aspect_ratio {
                                let aspect = img.original_height_px as f32 / img.original_width_px as f32;
                                img.height_mm = new_width * aspect;
                                self.image_height_input = format!("{:.1}", img.height_mm);
                            }
                            img.width_mm = new_width;
                            self.canvas.set_layout(self.layout.clone());
                            self.is_modified = true;
                        }
                    }
                }
            }
            Message::ImageHeightChanged(value) => {
                self.image_height_input = value.clone();
                if let Ok(new_height) = value.parse::<f32>() {
                    if new_height > 0.0 {
                        if let Some(img) = self.layout.selected_image_mut() {
                            if self.maintain_aspect_ratio {
                                let aspect = img.original_width_px as f32 / img.original_height_px as f32;
                                img.width_mm = new_height * aspect;
                                self.image_width_input = format!("{:.1}", img.width_mm);
                            }
                            img.height_mm = new_height;
                            self.canvas.set_layout(self.layout.clone());
                            self.is_modified = true;
                        }
                    }
                }
            }
            Message::MaintainAspectRatio(maintain) => {
                self.maintain_aspect_ratio = maintain;
            }
            Message::NewLayout => {
                self.layout = Layout::new();
                self.canvas.set_layout(self.layout.clone());
                self.current_file = None;
                self.project = None;
                self.is_modified = false;
                self.margin_top_input = "25.4".to_string();
                self.margin_bottom_input = "25.4".to_string();
                self.margin_left_input = "25.4".to_string();
                self.margin_right_input = "25.4".to_string();
            }
            Message::PrintersDiscovered(printers) => {
                self.printers = printers;
                if let Some(default_printer) = self.printers.iter().find(|p| p.is_default) {
                    self.selected_printer = Some(default_printer.name.clone());
                } else if let Some(first_printer) = self.printers.first() {
                    self.selected_printer = Some(first_printer.name.clone());
                }
            }
            Message::PrinterSelected(printer_name) => {
                self.selected_printer = Some(printer_name);
            }
            Message::PrintClicked => {
                if self.layout.images.is_empty() {
                    return Task::none();
                }
                let printer_name = match &self.selected_printer {
                    Some(name) => name.clone(),
                    None => return Task::none(),
                };
                
                // Set status to rendering
                self.print_status = PrintStatus::Rendering;
                
                let job = PrintJob {
                    layout: self.layout.clone(),
                    printer_name,
                    copies: self.print_copies,
                    dpi: self.print_dpi,
                };
                return Task::perform(
                    async move {
                        // Simulate brief delay to show the status
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        match execute_print_job(job) {
                            Ok(job_id) => Ok(job_id),
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    Message::PrintJobCompleted,
                );
            }
            Message::PrintJobCompleted(result) => {
                match result {
                    Ok(job_id) => {
                        log::info!("Print job submitted: {}", job_id);
                        self.print_status = PrintStatus::Completed(job_id);
                        
                        // Save the successful print settings
                        self.preferences.last_print_settings = config::LastPrintSettings {
                            printer_name: self.selected_printer.clone(),
                            paper_size: Some(self.layout.page.paper_size),
                            paper_type: Some(self.layout.page.paper_type),
                            print_quality: Some(self.layout.page.print_quality),
                            color_mode: Some(self.layout.page.color_mode),
                            orientation: Some(self.layout.page.orientation),
                            borderless: Some(self.layout.page.borderless),
                            copies: Some(self.print_copies),
                            margins: Some((
                                self.layout.page.margin_top_mm,
                                self.layout.page.margin_bottom_mm,
                                self.layout.page.margin_left_mm,
                                self.layout.page.margin_right_mm,
                            )),
                            last_success_time: Some(chrono::Utc::now()),
                        };
                        
                        // Save preferences to disk
                        if let Err(e) = self.config_manager.save_config(&self.preferences) {
                            log::error!("Failed to save print settings: {}", e);
                        } else {
                            log::info!("Saved successful print settings");
                        }
                    }
                    Err(error) => {
                        log::error!("Print job failed: {}", error);
                        self.print_status = PrintStatus::Failed(error);
                    }
                }
            }
            Message::DismissPrintStatus => {
                self.print_status = PrintStatus::Idle;
            }
            // File operations
            Message::SaveLayoutClicked => {
                if let Some(path) = &self.current_file {
                    // Save to existing file
                    return self.save_layout_to_file(path.clone());
                } else {
                    // No file yet, show save dialog
                    return Task::done(Message::SaveLayoutAs);
                }
            }
            Message::SaveLayoutAs => {
                let default_dir = self.preferences.last_open_directory.clone();
                return Task::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Print Layout", &["pxl"])
                            .set_title("Save Layout As")
                            .set_directory(default_dir.unwrap_or_else(|| PathBuf::from(".")))
                            .set_file_name("layout.pxl")
                            .save_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::LayoutSavePathSelected,
                );
            }
            Message::LayoutSavePathSelected(path) => {
                if let Some(path) = path {
                    return self.save_layout_to_file(path);
                }
            }
            Message::OpenLayoutClicked => {
                let default_dir = self.preferences.last_open_directory.clone();
                return Task::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Print Layout", &["pxl"])
                            .set_title("Open Layout")
                            .set_directory(default_dir.unwrap_or_else(|| PathBuf::from(".")))
                            .pick_file()
                            .await
                            .map(|f| f.path().to_path_buf())
                    },
                    Message::LayoutOpenPathSelected,
                );
            }
            Message::LayoutOpenPathSelected(path) => {
                if let Some(path) = path {
                    let config_manager = self.config_manager.clone();
                    return Task::perform(
                        async move {
                            match config_manager.load_layout(&path) {
                                Ok(project) => Ok(project),
                                Err(e) => Err(e.to_string()),
                            }
                        },
                        Message::LayoutLoaded,
                    );
                }
            }
            Message::LayoutLoaded(result) => {
                match result {
                    Ok(project) => {
                        self.layout = project.layout.clone();
                        self.canvas.set_layout(self.layout.clone());
                        self.project = Some(project);
                        self.is_modified = false;
                        
                        // Pre-populate thumbnail cache for loaded images
                        for item in &self.layout.images {
                            self.thumbnail_cache.entry(item.path.clone())
                                .or_insert_with(|| iced::widget::image::Handle::from_path(&item.path));
                        }
                        
                        // Update recent files
                        if let Some(path) = &self.current_file {
                            self.config_manager.add_recent_file(&mut self.preferences, path.clone());
                            let _ = self.config_manager.save_config(&self.preferences);
                        }
                        
                        log::info!("Layout loaded successfully");
                    }
                    Err(error) => {
                        log::error!("Failed to load layout: {}", error);
                    }
                }
            }
            Message::CheckAutoSave => {
                if self.config_manager.has_auto_save() {
                    log::info!("Auto-save file detected");
                    // Show recovery dialog to user
                    self.show_recovery_dialog = true;
                }
            }
            Message::RecoverAutoSave => {
                self.show_recovery_dialog = false;
                match self.config_manager.load_auto_save() {
                    Ok(project) => {
                        self.layout = project.layout.clone();
                        self.canvas.set_layout(self.layout.clone());
                        self.project = Some(project);
                        self.is_modified = true;
                        
                        // Pre-populate thumbnail cache for recovered images
                        for item in &self.layout.images {
                            self.thumbnail_cache.entry(item.path.clone())
                                .or_insert_with(|| iced::widget::image::Handle::from_path(&item.path));
                        }
                        
                        let _ = self.config_manager.delete_auto_save();
                        log::info!("Recovered from auto-save");
                    }
                    Err(e) => {
                        log::error!("Failed to recover auto-save: {}", e);
                    }
                }
            }
            Message::DiscardAutoSave => {
                self.show_recovery_dialog = false;
                let _ = self.config_manager.delete_auto_save();
                log::info!("Discarded auto-save");
            }
            Message::AutoSaveTick => {
                if self.preferences.auto_save_enabled && self.is_modified {
                    self.auto_save_counter += 1;
                    // Auto-save every N ticks (this would be time-based in real impl)
                    if self.auto_save_counter >= 10 {
                        let _ = self.config_manager.auto_save(&self.layout);
                        self.auto_save_counter = 0;
                    }
                }
                // Schedule next tick
                return Task::perform(
                    async {
                        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                    },
                    |_| Message::AutoSaveTick,
                );
            }
            Message::OpenRecentFile(path) => {
                self.show_recent_files_menu = false;
                // Check if file exists
                if path.exists() {
                    let path_clone = path.clone();
                    return Task::perform(
                        async move {
                            match std::fs::read_to_string(&path_clone) {
                                Ok(contents) => {
                                    match serde_json::from_str::<ProjectLayout>(&contents) {
                                        Ok(project) => Ok(project),
                                        Err(e) => Err(format!("Failed to parse layout: {}", e)),
                                    }
                                }
                                Err(e) => Err(format!("Failed to read file: {}", e)),
                            }
                        },
                        Message::LayoutLoaded,
                    );
                } else {
                    // Remove from recent files if it no longer exists
                    self.preferences.recent_files.retain(|p| p != &path);
                    let _ = self.config_manager.save_config(&self.preferences);
                    log::warn!("Recent file no longer exists: {:?}", path);
                }
            }
            Message::ToggleRecentFilesMenu => {
                self.show_recent_files_menu = !self.show_recent_files_menu;
            }
        }
        Task::none()
    }

    fn save_layout_to_file(&mut self, path: PathBuf) -> Task<Message> {
        // Create or update project
        let project = match &mut self.project {
            Some(proj) => {
                proj.layout = self.layout.clone();
                proj.update_modified();
                proj.clone()
            }
            None => {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unnamed")
                    .to_string();
                ProjectLayout::new(self.layout.clone(), name)
            }
        };

        // Save to file
        match self.config_manager.save_layout(&project, &path) {
            Ok(_) => {
                // Update recent files before setting current_file
                self.config_manager.add_recent_file(&mut self.preferences, path.clone());
                
                // Update last open directory
                if let Some(parent) = path.parent() {
                    self.preferences.last_open_directory = Some(parent.to_path_buf());
                }
                
                self.current_file = Some(path);
                self.project = Some(project);
                self.is_modified = false;
                
                let _ = self.config_manager.save_config(&self.preferences);
                log::info!("Layout saved successfully");
            }
            Err(e) => {
                log::error!("Failed to save layout: {}", e);
            }
        }
        
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        // ====================================================================
        // A: STORED SETTINGS AREA (Top bar with printer and file operations)
        // ====================================================================
        let printer_picker = if !self.printers.is_empty() {
            let printer_names: Vec<String> = self.printers.iter().map(|p| p.name.clone()).collect();
            pick_list(printer_names, self.selected_printer.clone(), Message::PrinterSelected)
                .width(Length::Fixed(200.0))
        } else {
            pick_list(vec!["No printers found".to_string()], Some("No printers found".to_string()), |_| Message::PrinterSelected("".to_string()))
                .width(Length::Fixed(200.0))
        };

        // Build recent files button with indicator
        let recent_btn_text = if self.preferences.recent_files.is_empty() {
            "Recent".to_string()
        } else {
            format!("Recent ({})", self.preferences.recent_files.len())
        };
        let recent_button = if self.preferences.recent_files.is_empty() {
            button(text(recent_btn_text).size(12))
        } else {
            button(text(recent_btn_text).size(12)).on_press(Message::ToggleRecentFilesMenu)
        };

        let stored_settings_area = row![
            text("Printer:").size(14),
            printer_picker,
            Space::with_width(Length::Fixed(20.0)),
            button("New").on_press(Message::NewLayout),
            button("Open").on_press(Message::OpenLayoutClicked),
            recent_button,
            button("Save").on_press(Message::SaveLayoutClicked),
            button("Save As").on_press(Message::SaveLayoutAs),
        ]
        .spacing(10)
        .padding(10)
        .align_y(Alignment::Center);

        // ====================================================================
        // D: TOOLS AREA (Toolbar with zoom, orientation, add/delete)
        // ====================================================================
        let delete_button = if self.layout.selected_image_id.is_some() {
            button(row![text("✕").size(14), text(" Delete").size(12)].align_y(Alignment::Center))
                .on_press(Message::DeleteImageClicked)
        } else {
            button(row![text("✕").size(14), text(" Delete").size(12)].align_y(Alignment::Center))
        };

        let orientation_btn = match self.layout.page.orientation {
            LayoutOrientation::Portrait => button(
                row![text("▯").size(16), text(" Portrait").size(12)].align_y(Alignment::Center)
            ).on_press(Message::OrientationToggled),
            LayoutOrientation::Landscape => button(
                row![text("▭").size(16), text(" Landscape").size(12)].align_y(Alignment::Center)
            ).on_press(Message::OrientationToggled),
        };

        let tools_area = row![
            button(row![text("+").size(16), text(" Add Image").size(12)].align_y(Alignment::Center))
                .on_press(Message::AddImageClicked),
            delete_button,
            Space::with_width(Length::Fixed(20.0)),
            button(text("−").size(18)).on_press(Message::ZoomOut),
            text(&self.zoom_text).size(14),
            button(text("+").size(18)).on_press(Message::ZoomIn),
            button(text("Fit").size(12)).on_press(Message::ZoomToFit),
            button(text("100%").size(12)).on_press(Message::ZoomReset),
            Space::with_width(Length::Fixed(20.0)),
            orientation_btn,
        ]
        .spacing(5)
        .padding(Padding::from([5, 10]))
        .align_y(Alignment::Center);

        // ====================================================================
        // C: SETTINGS AREA (Right sidebar with tabs)
        // ====================================================================
        let tab_buttons = row![
            button(text("Print").size(10))
                .on_press(Message::SettingsTabChanged(SettingsTab::PrintSettings))
                .style(if self.settings_tab == SettingsTab::PrintSettings { 
                    button::primary 
                } else { 
                    button::secondary 
                }),
            button(text("Layout").size(10))
                .on_press(Message::SettingsTabChanged(SettingsTab::Layout))
                .style(if self.settings_tab == SettingsTab::Layout { 
                    button::primary 
                } else { 
                    button::secondary 
                }),
            button(text("Image").size(10))
                .on_press(Message::SettingsTabChanged(SettingsTab::ImageTools))
                .style(if self.settings_tab == SettingsTab::ImageTools { 
                    button::primary 
                } else { 
                    button::secondary 
                }),
            button(text("Color").size(10))
                .on_press(Message::SettingsTabChanged(SettingsTab::ColorManagement))
                .style(if self.settings_tab == SettingsTab::ColorManagement { 
                    button::primary 
                } else { 
                    button::secondary 
                }),
        ]
        .spacing(2);

        let settings_content: Element<'_, Message> = match self.settings_tab {
            SettingsTab::PrintSettings => {
                // Print Settings Tab
                let paper_sizes = vec![
                    PaperSize::Photo3_5x5, PaperSize::Photo4x6, PaperSize::Photo5x5,
                    PaperSize::Photo5x7, PaperSize::Photo7x10, PaperSize::Photo8x10,
                    PaperSize::Letter, PaperSize::Legal, PaperSize::Photo10x12,
                    PaperSize::Photo11x17, PaperSize::Photo12x12, PaperSize::Photo13x19,
                    PaperSize::Panorama, PaperSize::A3, PaperSize::A4, PaperSize::A5,
                    PaperSize::Tabloid, PaperSize::Ledger,
                ];

                let paper_types = vec![
                    PaperType::Plain, PaperType::SuperHighGloss, PaperType::Glossy,
                    PaperType::SemiGloss, PaperType::Matte, PaperType::FineArt,
                ];

                let print_qualities = vec![
                    PrintQuality::Highest, PrintQuality::High,
                    PrintQuality::Standard, PrintQuality::Draft,
                ];

                column![
                    text("Media Type").size(12),
                    pick_list(paper_types, Some(self.layout.page.paper_type), Message::PaperTypeSelected)
                        .width(Length::Fill),
                    Space::with_height(Length::Fixed(10.0)),
                    text("Paper Size").size(12),
                    pick_list(paper_sizes, Some(self.layout.page.paper_size), Message::PaperSizeSelected)
                        .width(Length::Fill),
                    Space::with_height(Length::Fixed(10.0)),
                    checkbox("Borderless Printing", self.layout.page.borderless)
                        .on_toggle(Message::BorderlessToggled),
                    Space::with_height(Length::Fixed(10.0)),
                    text("Print Quality").size(12),
                    pick_list(print_qualities, Some(self.layout.page.print_quality), Message::PrintQualitySelected)
                        .width(Length::Fill),
                ]
                .spacing(5)
                .into()
            }
            SettingsTab::Layout => {
                // Layout Tab - Margins
                column![
                    text("Margins (mm)").size(12),
                    horizontal_rule(1),
                    row![
                        text("Top:").width(Length::Fixed(60.0)),
                        text_input("0", &self.margin_top_input)
                            .on_input(Message::MarginTopChanged)
                            .width(Length::Fixed(70.0)),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                    row![
                        text("Bottom:").width(Length::Fixed(60.0)),
                        text_input("0", &self.margin_bottom_input)
                            .on_input(Message::MarginBottomChanged)
                            .width(Length::Fixed(70.0)),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                    row![
                        text("Left:").width(Length::Fixed(60.0)),
                        text_input("0", &self.margin_left_input)
                            .on_input(Message::MarginLeftChanged)
                            .width(Length::Fixed(70.0)),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                    row![
                        text("Right:").width(Length::Fixed(60.0)),
                        text_input("0", &self.margin_right_input)
                            .on_input(Message::MarginRightChanged)
                            .width(Length::Fixed(70.0)),
                    ]
                    .spacing(5)
                    .align_y(Alignment::Center),
                    Space::with_height(Length::Fixed(15.0)),
                    text("Page Info").size(12),
                    horizontal_rule(1),
                    text(format!("Size: {:.1} × {:.1} mm", 
                        self.layout.page.width_mm, 
                        self.layout.page.height_mm)).size(11),
                    text(format!("Orientation: {}", self.layout.page.orientation)).size(11),
                ]
                .spacing(8)
                .into()
            }
            SettingsTab::ColorManagement => {
                // Color Management Tab
                let color_modes = vec![
                    ColorMode::UseICCProfile, ColorMode::DriverMatching,
                    ColorMode::NoColorCorrection, ColorMode::BlackAndWhite,
                ];

                column![
                    text("Color Mode").size(12),
                    pick_list(color_modes, Some(self.layout.page.color_mode), Message::ColorModeSelected)
                        .width(Length::Fill),
                    Space::with_height(Length::Fixed(15.0)),
                    text("Color Mode Info").size(11),
                    horizontal_rule(1),
                    match self.layout.page.color_mode {
                        ColorMode::UseICCProfile => text("Uses ICC profiles for accurate color matching with your paper type.").size(10),
                        ColorMode::DriverMatching => text("Uses driver color matching for consistent results.").size(10),
                        ColorMode::NoColorCorrection => text("Prints without any color correction applied.").size(10),
                        ColorMode::BlackAndWhite => text("Converts image to black and white for printing.").size(10),
                    },
                ]
                .spacing(8)
                .into()
            }
            SettingsTab::ImageTools => {
                // Image Tools Tab
                if self.layout.selected_image_id.is_some() {
                    let selected_img = self.layout.selected_image();
                    let (rotation_text, flip_h, flip_v) = if let Some(img) = selected_img {
                        (format!("{}°", img.rotation_degrees), img.flip_horizontal, img.flip_vertical)
                    } else {
                        ("0°".to_string(), false, false)
                    };

                    column![
                        text("Rotation").size(12),
                        row![
                            text(format!("Current: {}", rotation_text)).size(10),
                        ],
                        row![
                            button(text("↺ 90°").size(10))
                                .on_press(Message::RotateImageCCW)
                                .padding(5),
                            button(text("↻ 90°").size(10))
                                .on_press(Message::RotateImageCW)
                                .padding(5),
                        ]
                        .spacing(5),
                        Space::with_height(Length::Fixed(10.0)),
                        text("Flip").size(12),
                        row![
                            button(text(if flip_h { "↔ H ✓" } else { "↔ H" }).size(10))
                                .on_press(Message::FlipImageHorizontal)
                                .style(if flip_h { button::primary } else { button::secondary })
                                .padding(5),
                            button(text(if flip_v { "↕ V ✓" } else { "↕ V" }).size(10))
                                .on_press(Message::FlipImageVertical)
                                .style(if flip_v { button::primary } else { button::secondary })
                                .padding(5),
                        ]
                        .spacing(5),
                        Space::with_height(Length::Fixed(10.0)),
                        text("Size (mm)").size(12),
                        row![
                            text("W:").size(10).width(Length::Fixed(20.0)),
                            text_input("0", &self.image_width_input)
                                .on_input(Message::ImageWidthChanged)
                                .width(Length::Fixed(55.0)),
                            text("H:").size(10).width(Length::Fixed(20.0)),
                            text_input("0", &self.image_height_input)
                                .on_input(Message::ImageHeightChanged)
                                .width(Length::Fixed(55.0)),
                        ]
                        .spacing(3)
                        .align_y(Alignment::Center),
                        checkbox("Maintain aspect ratio", self.maintain_aspect_ratio)
                            .on_toggle(Message::MaintainAspectRatio)
                            .size(14),
                        Space::with_height(Length::Fixed(10.0)),
                        text("Opacity").size(12),
                        row![
                            text_input("100", &self.image_opacity_input)
                                .on_input(Message::ImageOpacityChanged)
                                .width(Length::Fixed(50.0)),
                            text("%").size(10),
                        ]
                        .spacing(3)
                        .align_y(Alignment::Center),
                    ]
                    .spacing(5)
                    .into()
                } else {
                    column![
                        text("No Image Selected").size(12),
                        Space::with_height(Length::Fixed(10.0)),
                        text("Select an image from the\nthumbnails below to edit\nits properties.").size(10),
                    ]
                    .spacing(5)
                    .into()
                }
            }
        };

        let settings_panel = column![
            text("Settings").size(14),
            horizontal_rule(1),
            tab_buttons,
            Space::with_height(Length::Fixed(10.0)),
            scrollable(settings_content).height(Length::Fill),
        ]
        .spacing(5)
        .padding(10)
        .width(Length::Fixed(220.0));

        // ====================================================================
        // A: PREVIEW AREA (Center - Canvas with scrollbars)
        // ====================================================================
        // Calculate canvas size based on page dimensions and zoom
        let canvas_width = self.canvas.mm_to_pixels(self.layout.page.width_mm) + 40.0;
        let canvas_height = self.canvas.mm_to_pixels(self.layout.page.height_mm) + 40.0;
        
        let canvas_elem: Element<'_, CanvasMessage> = canvas(&self.canvas)
            .width(Length::Fixed(canvas_width))
            .height(Length::Fixed(canvas_height))
            .into();
        let canvas_widget = canvas_elem.map(Message::CanvasMessage);
        
        // Wrap canvas in a container with padding for visual margin
        let canvas_container = container(canvas_widget)
            .padding(20)
            .style(container::bordered_box);

        // Wrap in scrollable for both directions
        let preview_area = scrollable(
            scrollable(canvas_container)
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::default()
                ))
        )
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::default()
        ))
        .width(Length::Fill)
        .height(Length::Fill);

        // ====================================================================
        // E: THUMBNAILS AREA (Bottom with image thumbnails)
        // ====================================================================
        let thumbnails: Vec<Element<'_, Message>> = self.layout.images.iter().map(|img| {
            let filename = img.path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("?");
            
            // Truncate filename if too long
            let display_name = if filename.len() > 12 {
                format!("{}...", &filename[..9])
            } else {
                filename.to_string()
            };
            
            let is_selected = self.layout.selected_image_id.as_ref() == Some(&img.id);
            let style = if is_selected { button::primary } else { button::secondary };
            
            // Use cached thumbnail handle or create from path
            let img_handle = self.thumbnail_cache
                .get(&img.path)
                .cloned()
                .unwrap_or_else(|| iced::widget::image::Handle::from_path(&img.path));
            
            let thumb_image = iced_image(img_handle)
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0));
            
            let thumb_btn = button(
                column![
                    thumb_image,
                    text(display_name).size(9),
                ]
                .align_x(Alignment::Center)
                .spacing(2)
            )
            .on_press(Message::ThumbnailClicked(img.id.clone()))
            .style(style)
            .padding(5);

            thumb_btn.into()
        }).collect();

        let thumbnails_row = if thumbnails.is_empty() {
            row![text("No images. Click 'Add Image' to add photos.").size(12)]
                .spacing(10)
                .padding(10)
        } else {
            let mut r = row![].spacing(10).padding(10);
            for thumb in thumbnails {
                r = r.push(thumb);
            }
            r
        };

        let thumbnails_area = column![
            row![
                text("Thumbnails").size(12),
                Space::with_width(Length::Fill),
                text(format!("{} image(s)", self.layout.images.len())).size(11),
            ]
            .padding(Padding::from([5, 10])),
            scrollable(thumbnails_row).direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default()
            )),
        ]
        .height(Length::Fixed(120.0));

        // ====================================================================
        // F: PRINT BUTTON AREA (Bottom right)
        // ====================================================================
        let print_button = if self.selected_printer.is_some() && !self.layout.images.is_empty() {
            button(text("Print").size(16))
                .on_press(Message::PrintClicked)
                .padding(Padding::from([10, 30]))
        } else {
            button(text("Print").size(16))
                .padding(Padding::from([10, 30]))
        };

        let print_area = row![
            text("Copies:").size(12),
            text_input("1", &self.copies_input)
                .on_input(Message::CopiesChanged)
                .width(Length::Fixed(50.0)),
            Space::with_width(Length::Fixed(20.0)),
            print_button,
        ]
        .spacing(10)
        .padding(10)
        .align_y(Alignment::Center);

        // ====================================================================
        // ASSEMBLE THE LAYOUT
        // ====================================================================
        // Top section: Stored settings
        // Middle section: Tools + Preview + Settings
        // Bottom section: Thumbnails + Print button

        let middle_section = row![
            column![
                preview_area,
            ]
            .width(Length::Fill)
            .height(Length::Fill),
            vertical_rule(1),
            settings_panel,
        ];

        let bottom_section = row![
            container(thumbnails_area).width(Length::Fill),
            vertical_rule(1),
            print_area,
        ]
        .height(Length::Fixed(120.0));

        let main_content = column![
            stored_settings_area,
            horizontal_rule(1),
            tools_area,
            horizontal_rule(1),
            middle_section,
            horizontal_rule(1),
            bottom_section,
        ];

        let base = container(main_content)
            .width(Length::Fill)
            .height(Length::Fill);

        // Create the base with optional overlays
        let dark_text = Color::from_rgb(0.1, 0.1, 0.1);
        
        // First, check if we need to show the recovery dialog
        if self.show_recovery_dialog {
            let modal_content = container(
                column![
                    text("Recover Unsaved Work?").size(20).color(dark_text),
                    Space::with_height(Length::Fixed(15.0)),
                    text("An auto-save file was found from a previous session.").size(14).color(Color::from_rgb(0.3, 0.3, 0.3)),
                    text("Would you like to recover it?").size(14).color(Color::from_rgb(0.3, 0.3, 0.3)),
                    Space::with_height(Length::Fixed(20.0)),
                    row![
                        button(text("Recover").size(14))
                            .on_press(Message::RecoverAutoSave)
                            .padding(Padding::from([10, 30])),
                        Space::with_width(Length::Fixed(20.0)),
                        button(text("Discard").size(14))
                            .on_press(Message::DiscardAutoSave)
                            .style(button::secondary)
                            .padding(Padding::from([10, 30])),
                    ]
                    .spacing(10),
                ]
                .align_x(Alignment::Center)
                .spacing(5)
            )
            .padding(40)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::WHITE)),
                border: iced::Border {
                    color: Color::from_rgb(0.3, 0.5, 0.8),
                    width: 3.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            });

            return iced::widget::stack![
                base,
                opaque(
                    mouse_area(
                        center(modal_content)
                            .style(|_theme| container::Style {
                                background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                                ..Default::default()
                            })
                    )
                )
            ]
            .into();
        }
        
        // Show recent files popup if toggled
        if self.show_recent_files_menu && !self.preferences.recent_files.is_empty() {
            let recent_items: Vec<Element<'_, Message>> = self.preferences.recent_files
                .iter()
                .take(10)
                .map(|path| {
                    let display_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown");
                    let path_clone = path.clone();
                    button(text(display_name).size(12))
                        .width(Length::Fill)
                        .on_press(Message::OpenRecentFile(path_clone))
                        .style(button::text)
                        .into()
                })
                .collect();
            
            let popup_content = container(
                column(recent_items)
                    .spacing(2)
                    .width(Length::Fixed(250.0))
            )
            .padding(10)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::WHITE)),
                border: iced::Border {
                    color: Color::from_rgb(0.7, 0.7, 0.7),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            });

            // Position the popup near the top-left where the buttons are
            let popup_positioned = container(
                column![
                    Space::with_height(Length::Fixed(50.0)), // Offset from top
                    row![
                        Space::with_width(Length::Fixed(400.0)), // Offset from left to align with Recent button
                        popup_content,
                    ],
                ]
            )
            .width(Length::Fill)
            .height(Length::Fill);

            return iced::widget::stack![
                base,
                mouse_area(popup_positioned)
                    .on_press(Message::ToggleRecentFilesMenu)
            ]
            .into();
        }

        // Show modal overlay when printing
        match &self.print_status {
            PrintStatus::Idle => base.into(),
            PrintStatus::Rendering => {
                let modal_content = container(
                    column![
                        text("PRINTING").size(24).color(dark_text),
                        Space::with_height(Length::Fixed(15.0)),
                        text("[  ]  Rendering...").size(16).color(dark_text),
                        Space::with_height(Length::Fixed(20.0)),
                        progress_bar(0.0..=100.0, 30.0)
                            .width(Length::Fixed(250.0))
                            .height(Length::Fixed(12.0)),
                        Space::with_height(Length::Fixed(15.0)),
                        text("Please wait...").size(14).color(Color::from_rgb(0.4, 0.4, 0.4)),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(5)
                )
                .padding(40)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::WHITE)),
                    border: iced::Border {
                        color: Color::from_rgb(0.3, 0.5, 0.8),
                        width: 3.0,
                        radius: 12.0.into(),
                    },
                    ..Default::default()
                });

                iced::widget::stack![
                    base,
                    opaque(
                        mouse_area(
                            center(modal_content)
                                .style(|_theme| container::Style {
                                    background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                                    ..Default::default()
                                })
                        )
                    )
                ]
                .into()
            }
            PrintStatus::Sending => {
                let modal_content = container(
                    column![
                        text("PRINTING").size(24).color(dark_text),
                        Space::with_height(Length::Fixed(15.0)),
                        text("[>>]  Sending to printer...").size(16).color(dark_text),
                        Space::with_height(Length::Fixed(20.0)),
                        progress_bar(0.0..=100.0, 70.0)
                            .width(Length::Fixed(250.0))
                            .height(Length::Fixed(12.0)),
                        Space::with_height(Length::Fixed(15.0)),
                        text("Please wait...").size(14).color(Color::from_rgb(0.4, 0.4, 0.4)),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(5)
                )
                .padding(40)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::WHITE)),
                    border: iced::Border {
                        color: Color::from_rgb(0.3, 0.5, 0.8),
                        width: 3.0,
                        radius: 12.0.into(),
                    },
                    ..Default::default()
                });

                iced::widget::stack![
                    base,
                    opaque(
                        mouse_area(
                            center(modal_content)
                                .style(|_theme| container::Style {
                                    background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                                    ..Default::default()
                                })
                        )
                    )
                ]
                .into()
            }
            PrintStatus::Completed(job_id) => {
                let modal_content = container(
                    column![
                        text("[OK]").size(36).color(Color::from_rgb(0.2, 0.7, 0.3)),
                        Space::with_height(Length::Fixed(15.0)),
                        text("Print Job Sent Successfully!").size(18).color(dark_text),
                        Space::with_height(Length::Fixed(10.0)),
                        text(format!("Job ID: {}", job_id)).size(13).color(Color::from_rgb(0.4, 0.4, 0.4)),
                        Space::with_height(Length::Fixed(20.0)),
                        button(text("OK").size(14))
                            .on_press(Message::DismissPrintStatus)
                            .padding(Padding::from([10, 40])),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(5)
                )
                .padding(40)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::WHITE)),
                    border: iced::Border {
                        color: Color::from_rgb(0.2, 0.7, 0.3),
                        width: 3.0,
                        radius: 12.0.into(),
                    },
                    ..Default::default()
                });

                iced::widget::stack![
                    base,
                    opaque(
                        mouse_area(
                            center(modal_content)
                                .style(|_theme| container::Style {
                                    background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                                    ..Default::default()
                                })
                        )
                    )
                ]
                .into()
            }
            PrintStatus::Failed(error) => {
                let modal_content = container(
                    column![
                        text("[!!]").size(36).color(Color::from_rgb(0.9, 0.3, 0.3)),
                        Space::with_height(Length::Fixed(15.0)),
                        text("Print Job Failed").size(18).color(dark_text),
                        Space::with_height(Length::Fixed(10.0)),
                        text(error).size(13).color(Color::from_rgb(0.5, 0.3, 0.3)),
                        Space::with_height(Length::Fixed(20.0)),
                        button(text("OK").size(14))
                            .on_press(Message::DismissPrintStatus)
                            .padding(Padding::from([10, 40])),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(5)
                )
                .padding(40)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::WHITE)),
                    border: iced::Border {
                        color: Color::from_rgb(0.9, 0.3, 0.3),
                        width: 3.0,
                        radius: 12.0.into(),
                    },
                    ..Default::default()
                });

                iced::widget::stack![
                    base,
                    opaque(
                        mouse_area(
                            center(modal_content)
                                .style(|_theme| container::Style {
                                    background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                                    ..Default::default()
                                })
                        )
                    )
                ]
                .into()
            }
        }
    }

    pub fn title(&self) -> String {
        let base_title = match &self.current_file {
            Some(path) => {
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unnamed");
                format!("Print Layout - {}", filename)
            }
            None => "Print Layout".to_string(),
        };
        
        if self.is_modified {
            format!("{}*", base_title)
        } else {
            base_title
        }
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}
