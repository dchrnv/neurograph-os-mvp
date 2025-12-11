// Screen management
pub mod auth;
pub mod dashboard;
pub mod chat;

use iced::Element;
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Auth,
    Dashboard,
    Chat,
    Modules,
    Logs,
    Settings,
    Integrations,
}

impl Screen {
    pub fn view<Message: 'static + Clone>(&self, theme: &Theme) -> Element<'static, Message> {
        match self {
            Screen::Auth => auth::view(theme),
            Screen::Dashboard => dashboard::view(theme),
            Screen::Chat => chat::view(theme, chat::ChatMode::Chat, ""),
            _ => placeholder_view(),
        }
    }
}

fn placeholder_view<Message: 'static + Clone>() -> Element<'static, Message> {
    use iced::widget::{container, text};

    container(text("Coming soon..."))
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center(iced::Length::Fill)
        .into()
}
