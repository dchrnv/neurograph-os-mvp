// NeuroGraph Desktop UI v0.46.0
// Native desktop interface for cognitive architecture

mod theme;
mod screens;
mod components;
mod core_bridge;
mod utils;

use iced::{Element, Task, Size, window};
use screens::Screen;
use theme::Theme;

fn main() -> iced::Result {
    iced::application("NeuroGraph v0.46.0", NeuroGraphApp::update, NeuroGraphApp::view)
        .window(window::Settings {
            size: Size::new(1280.0, 800.0),
            min_size: Some(Size::new(1024.0, 768.0)),
            ..Default::default()
        })
        .theme(NeuroGraphApp::theme)
        .run_with(NeuroGraphApp::new)
}

struct NeuroGraphApp {
    theme: Theme,
    current_screen: Screen,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    ScreenChanged(Screen),
}

impl NeuroGraphApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                theme: Theme::default(),
                // TODO: Start with Auth, switch to Dashboard after PIN
                current_screen: Screen::Dashboard, // Temporary: testing Dashboard
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::ScreenChanged(screen) => {
                self.current_screen = screen;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        self.current_screen.view(&self.theme)
    }

    fn theme(&self) -> iced::Theme {
        match self.theme {
            Theme::Dark => iced::Theme::Dark,
            Theme::Light => iced::Theme::Light,
        }
    }
}
