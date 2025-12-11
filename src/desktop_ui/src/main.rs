// NeuroGraph Desktop UI v0.46.0
// Native desktop interface for cognitive architecture

mod theme;
mod screens;
mod components;
mod core_bridge;
mod utils;

use iced::{Element, Task, Size, window};
use screens::{Screen, chat::{ChatMode, ChatMessage}};
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
    chat_mode: ChatMode,
    chat_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    ScreenChanged(Screen),
    Chat(ChatMessage),
}

impl NeuroGraphApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                theme: Theme::default(),
                // TODO: Start with Auth, switch to Dashboard after PIN
                current_screen: Screen::Chat, // Temporary: testing Chat
                chat_mode: ChatMode::Chat,
                chat_input: String::new(),
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
            Message::Chat(chat_msg) => {
                match chat_msg {
                    ChatMessage::InputChanged(input) => {
                        self.chat_input = input;
                    }
                    ChatMessage::Send => {
                        // TODO: Send message/command to core via FFI bridge
                        println!("[{}] Sending: {}",
                            match self.chat_mode {
                                ChatMode::Chat => "CHAT",
                                ChatMode::Terminal => "CMD",
                            },
                            self.chat_input
                        );
                        self.chat_input.clear();
                    }
                    ChatMessage::ToggleMode => {
                        self.chat_mode = match self.chat_mode {
                            ChatMode::Chat => ChatMode::Terminal,
                            ChatMode::Terminal => ChatMode::Chat,
                        };
                        println!("Mode switched to: {:?}", self.chat_mode);
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match self.current_screen {
            Screen::Chat => {
                screens::chat::view(&self.theme, self.chat_mode, &self.chat_input)
                    .map(Message::Chat)
            }
            _ => {
                let screen_view: Element<()> = self.current_screen.view(&self.theme);
                screen_view.map(|_| Message::ScreenChanged(self.current_screen))
            }
        }
    }

    fn theme(&self) -> iced::Theme {
        match self.theme {
            Theme::Dark => iced::Theme::Dark,
            Theme::Light => iced::Theme::Light,
        }
    }
}
