use iced::widget::{
    button, canvas, column, container, pick_list, row, text, text_input,
};
use iced::{executor, Length};
use iced::{Application, Command, Element, Settings, Size, Theme};
use image::GenericImageView;
use std::path::PathBuf;

mod canvas_widget;
mod layout;
mod printing;

use canvas_widget::{CanvasMessage, LayoutCanvas};
use layout::{Layout, PaperSize, PlacedImage};
use printing::{discover_printers, execute_print_job, Orientation, PrintJob, PrinterInfo};

pub fn main() -> iced::Result {
    env_logger::init();
    PrintLayout::run(Settings {
        window: iced::window::Settings {
            size: Size::new(1200.0, 800.0),
            min_size: Some(Size::new(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub enum Message {
    CanvasMessage(CanvasMessage),
    AddImageClicked,
    ImageFilesSelected(Vec<PathBuf>),
    DeleteImageClicked,
    PaperSizeSelected(PaperSize),
    MarginTopChanged(String),
    MarginBottomChanged(String),
    MarginLeftChanged(String),
    MarginRightChanged(String),
    ZoomIn,
    ZoomOut,
    ZoomReset,
    // Printing messages
    PrintersDiscovered(Vec<PrinterInfo>),
    PrinterSelected(String),
    PrintClicked,
    PrintJobCompleted(Result<String, String>),
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
    dragging: bool,
    drag_start_pos: (f32, f32),         // Initial click position in mm
    drag_image_initial_pos: (f32, f32), // Initial image position in mm
    // Printing state
    printers: Vec<PrinterInfo>,
    selected_printer: Option<String>,
    print_copies: u32,
    print_dpi: u32,
}

impl Application for PrintLayout {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        log::info!("Initializing Print Layout v{}", VERSION);

        let layout = Layout::new();
        let canvas = LayoutCanvas::new(layout.clone());

        (
            PrintLayout {
                layout,
                canvas,
                zoom: 1.0,
                margin_top_input: "25.4".to_string(),
                margin_bottom_input: "25.4".to_string(),
                margin_left_input: "25.4".to_string(),
                margin_right_input: "25.4".to_string(),
                dragging: false,
                drag_start_pos: (0.0, 0.0),
                drag_image_initial_pos: (0.0, 0.0),
                printers: Vec::new(),
                selected_printer: None,
                print_copies: 1,
                print_dpi: 300,
            },
            // Discover printers on startup
            Command::perform(
                async {
                    discover_printers().unwrap_or_else(|e| {
                        log::error!("Failed to discover printers: {}", e);
                        Vec::new()
                    })
                },
                Message::PrintersDiscovered,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Print Layout")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CanvasMessage(canvas_msg) => match canvas_msg {
                CanvasMessage::SelectImage(id) => {
                    log::info!("Selected image: {}", id);
                    self.layout.selected_image_id = Some(id.clone());

                    // Start dragging - record initial positions
                    if let Some(image) = self.layout.get_image(&id) {
                        self.dragging = true;
                        self.drag_image_initial_pos = (image.x_mm, image.y_mm);
                        self.drag_start_pos = (0.0, 0.0); // Will be set on first mouse move
                    }

                    self.canvas.set_layout(self.layout.clone());
                }
                CanvasMessage::DeselectAll => {
                    log::info!("Deselected all images");
                    self.layout.selected_image_id = None;
                    self.dragging = false;
                    self.canvas.set_layout(self.layout.clone());
                }
                CanvasMessage::MouseMoved(x, y) => {
                    // Handle mouse movement for dragging
                    if self.dragging {
                        if let Some(id) = self.layout.selected_image_id.clone() {
                            // If this is the first move, record the start position
                            if self.drag_start_pos == (0.0, 0.0) {
                                self.drag_start_pos = (x, y);
                            }

                            // Calculate delta from start position
                            let dx = x - self.drag_start_pos.0;
                            let dy = y - self.drag_start_pos.1;

                            // Update image position
                            let new_x = self.drag_image_initial_pos.0 + dx;
                            let new_y = self.drag_image_initial_pos.1 + dy;

                            if let Some(image) = self.layout.get_image_mut(&id) {
                                image.x_mm = new_x;
                                image.y_mm = new_y;
                                self.canvas.set_layout(self.layout.clone());
                            }
                        }
                    }
                }
                CanvasMessage::MouseReleased => {
                    // Stop dragging
                    if self.dragging {
                        log::debug!("Drag ended");
                        self.dragging = false;
                        self.drag_start_pos = (0.0, 0.0);
                    }
                }
                CanvasMessage::ImageMoved(id, x, y) => {
                    if let Some(image) = self.layout.get_image_mut(&id) {
                        image.x_mm = x;
                        image.y_mm = y;
                        self.canvas.set_layout(self.layout.clone());
                        log::debug!("Image {} moved to ({:.1}, {:.1})", id, x, y);
                    }
                }
                CanvasMessage::ImageResized(id, width, height) => {
                    if let Some(image) = self.layout.get_image_mut(&id) {
                        image.width_mm = width;
                        image.height_mm = height;
                        self.canvas.set_layout(self.layout.clone());
                    }
                }
                CanvasMessage::CanvasClicked(x, y) => {
                    log::debug!("Canvas clicked at: ({}, {})", x, y);
                }
            },
            Message::AddImageClicked => {
                log::info!("Opening file dialog to add images");
                return Command::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("Images", &["png", "jpg", "jpeg", "gif", "bmp", "webp"])
                            .set_title("Select Images to Add")
                            .pick_files()
                            .await
                            .map(|files| {
                                files
                                    .into_iter()
                                    .map(|f| f.path().to_path_buf())
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default()
                    },
                    Message::ImageFilesSelected,
                );
            }
            Message::ImageFilesSelected(paths) => {
                log::info!("Loading {} image(s)", paths.len());
                for path in paths {
                    match image::open(&path) {
                        Ok(img) => {
                            let (width, height) = img.dimensions();
                            let placed_image = PlacedImage::new(path.clone(), width, height);
                            self.layout.add_image(placed_image);
                            log::info!("Added image: {} ({}x{})", path.display(), width, height);
                        }
                        Err(e) => {
                            log::error!("Failed to load image {}: {}", path.display(), e);
                        }
                    }
                }
                self.canvas.set_layout(self.layout.clone());
            }
            Message::DeleteImageClicked => {
                if let Some(id) = &self.layout.selected_image_id.clone() {
                    log::info!("Deleting image: {}", id);
                    self.layout.remove_image(id);
                    self.canvas.set_layout(self.layout.clone());
                }
            }
            Message::PaperSizeSelected(paper_size) => {
                log::info!("Paper size changed to: {:?}", paper_size);
                let (width, height) = paper_size.to_dimensions();
                self.layout.page.width_mm = width;
                self.layout.page.height_mm = height;
                self.layout.page.paper_size = paper_size;
                self.canvas.set_layout(self.layout.clone());
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
                self.canvas.set_zoom(self.zoom);
                log::info!("Zoom in: {:.0}%", self.zoom * 100.0);
            }
            Message::ZoomOut => {
                self.zoom = (self.zoom / 1.2).max(0.1);
                self.canvas.set_zoom(self.zoom);
                log::info!("Zoom out: {:.0}%", self.zoom * 100.0);
            }
            Message::ZoomReset => {
                self.zoom = 1.0;
                self.canvas.set_zoom(self.zoom);
                log::info!("Zoom reset: 100%");
            }
            Message::PrintersDiscovered(printers) => {
                log::info!("Discovered {} printers", printers.len());
                self.printers = printers;
                // Select default printer if available
                if let Some(default_printer) = self.printers.iter().find(|p| p.is_default) {
                    self.selected_printer = Some(default_printer.name.clone());
                    log::info!("Selected default printer: {}", default_printer.name);
                } else if let Some(first_printer) = self.printers.first() {
                    self.selected_printer = Some(first_printer.name.clone());
                    log::info!("Selected first printer: {}", first_printer.name);
                }
            }
            Message::PrinterSelected(printer_name) => {
                log::info!("Printer selected: {}", printer_name);
                self.selected_printer = Some(printer_name);
            }
            Message::PrintClicked => {
                log::info!("Print button clicked");

                // Validate layout has images
                if self.layout.images.is_empty() {
                    log::warn!("Cannot print: no images on layout");
                    // TODO: Show error notification
                    return Command::none();
                }

                // Validate printer is selected
                let printer_name = match &self.selected_printer {
                    Some(name) => name.clone(),
                    None => {
                        log::warn!("Cannot print: no printer selected");
                        // TODO: Show error notification
                        return Command::none();
                    }
                };

                // Create print job
                let job = PrintJob {
                    layout: self.layout.clone(),
                    printer_name,
                    copies: self.print_copies,
                    dpi: self.print_dpi,
                    orientation: Orientation::Portrait,
                };

                // Execute print job in background
                return Command::perform(
                    async move {
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
                        log::info!("Print job submitted successfully: {}", job_id);
                        // TODO: Show success notification
                    }
                    Err(error) => {
                        log::error!("Print job failed: {}", error);
                        // TODO: Show error notification
                    }
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // Paper size options
        let paper_sizes = vec![
            PaperSize::A4,
            PaperSize::A3,
            PaperSize::A5,
            PaperSize::Letter,
            PaperSize::Legal,
            PaperSize::Tabloid,
        ];

        // Left panel - Page Settings
        let mut page_settings_column = column![
            text("Page Settings").size(16),
            text("Paper Size:").size(12),
            pick_list(
                paper_sizes,
                Some(self.layout.page.paper_size),
                Message::PaperSizeSelected
            ),
            text("Margins (mm):").size(12),
            row![
                text("Top:"),
                text_input("25.4", &self.margin_top_input)
                    .on_input(Message::MarginTopChanged)
                    .width(Length::Fixed(60.0)),
            ]
            .spacing(5),
            row![
                text("Bottom:"),
                text_input("25.4", &self.margin_bottom_input)
                    .on_input(Message::MarginBottomChanged)
                    .width(Length::Fixed(60.0)),
            ]
            .spacing(5),
            row![
                text("Left:"),
                text_input("25.4", &self.margin_left_input)
                    .on_input(Message::MarginLeftChanged)
                    .width(Length::Fixed(60.0)),
            ]
            .spacing(5),
            row![
                text("Right:"),
                text_input("25.4", &self.margin_right_input)
                    .on_input(Message::MarginRightChanged)
                    .width(Length::Fixed(60.0)),
            ]
            .spacing(5),
        ]
        .spacing(10);

        // Add printer selection if printers are available
        if !self.printers.is_empty() {
            page_settings_column = page_settings_column.push(text("Printer:").size(12));

            let printer_names: Vec<String> = self.printers.iter().map(|p| p.name.clone()).collect();
            let selected_printer_name = self.selected_printer.clone();

            page_settings_column = page_settings_column.push(pick_list(
                printer_names,
                selected_printer_name,
                Message::PrinterSelected,
            ));
        }

        let page_settings = page_settings_column.padding(10).width(Length::Fixed(200.0));

        // Toolbar
        let delete_button = if self.layout.selected_image_id.is_some() {
            button("Delete Image").on_press(Message::DeleteImageClicked)
        } else {
            button("Delete Image")
        };

        // Print button - only enabled if printer is selected and images exist
        let print_button = if self.selected_printer.is_some() && !self.layout.images.is_empty() {
            button("Print").on_press(Message::PrintClicked)
        } else {
            button("Print")
        };

        let toolbar = row![
            button("Add Image").on_press(Message::AddImageClicked),
            delete_button,
            print_button,
            button("Zoom In").on_press(Message::ZoomIn),
            button("Zoom Out").on_press(Message::ZoomOut),
            button("100%").on_press(Message::ZoomReset),
        ]
        .spacing(10)
        .padding(10);

        // Canvas
        let canvas_elem: Element<'_, CanvasMessage> = canvas(&self.canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        let canvas_widget = canvas_elem.map(Message::CanvasMessage);

        // Main content area - for now just use canvas
        // TODO: Implement image widget overlays when Iced provides better positioning APIs
        let main_area = column![toolbar, canvas_widget].spacing(0);

        // Combine with sidebar
        let content_with_sidebar = row![page_settings, main_area].spacing(0);

        // Status bar
        let printer_status = if let Some(printer_name) = &self.selected_printer {
            format!("Printer: {}", printer_name)
        } else if self.printers.is_empty() {
            "No printers found".to_string()
        } else {
            "No printer selected".to_string()
        };

        let status = row![
            text(format!("Images: {}", self.layout.images.len())),
            text(format!("Zoom: {:.0}%", self.zoom * 100.0)),
            text(format!("Paper: {:?}", self.layout.page.paper_size)),
            text(printer_status),
            text(format!("Version {}", VERSION)),
        ]
        .spacing(20)
        .padding(10);

        // Main layout
        let content = column![content_with_sidebar, status].spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::default()
    }
}
