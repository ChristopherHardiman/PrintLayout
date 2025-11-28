use iced::widget::{button, canvas, column, container, row, text};
use iced::{executor, Length};
use iced::{Application, Command, Element, Settings, Size, Theme};

mod canvas_widget;
mod layout;

use canvas_widget::{CanvasMessage, LayoutCanvas};
use layout::Layout;

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
    ZoomIn,
    ZoomOut,
    ZoomReset,
}

struct PrintLayout {
    layout: Layout,
    canvas: LayoutCanvas,
    zoom: f32,
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
            },
            Command::none(),
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
                    self.layout.selected_image_id = Some(id);
                    self.canvas.set_layout(self.layout.clone());
                }
                CanvasMessage::DeselectAll => {
                    log::info!("Deselected all images");
                    self.layout.selected_image_id = None;
                    self.canvas.set_layout(self.layout.clone());
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
                CanvasMessage::CanvasClicked(x, y) => {
                    log::debug!("Canvas clicked at: ({}, {})", x, y);
                }
            },
            Message::AddImageClicked => {
                log::info!("Add Image clicked (file dialog not yet implemented)");
                // TODO: Implement file dialog in Phase 3
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
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        // Toolbar
        let toolbar = row![
            button("Add Image").on_press(Message::AddImageClicked),
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

        // Status bar
        let status = row![
            text(format!("Images: {}", self.layout.images.len())),
            text(format!("Zoom: {:.0}%", self.zoom * 100.0)),
            text(format!("Paper: {:?}", self.layout.page.paper_size)),
            text(format!("Version {}", VERSION)),
        ]
        .spacing(20)
        .padding(10);

        // Main layout
        let content = column![toolbar, canvas_widget, status].spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::default()
    }
}
