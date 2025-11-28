use iced::widget::{column, container, text};
use iced::{executor, Length};
use iced::{Application, Command, Element, Settings, Size, Theme};

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
    // Messages will be expanded in later phases
}

struct PrintLayout {
    // State will be expanded in later phases
}

impl Application for PrintLayout {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        log::info!("Initializing Print Layout v{}", VERSION);

        (
            PrintLayout {
                // Initialize with default values
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Print Layout")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = column![
            text("Print Layout").size(32),
            text("Ready").size(16),
            text(format!("Version {}", VERSION)).size(12),
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::default()
    }
}
