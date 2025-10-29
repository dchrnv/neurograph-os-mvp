use iced::{Application, Command, Element, Theme};

use crate::auth::AuthState;
use crate::core::CoreBridge;
use crate::workspaces::{Workspace, WorkspaceView};

pub struct NeuroGraphApp {
    auth_state: AuthState,
    current_workspace: Workspace,
    core: CoreBridge,  // Прямой доступ к Rust core!
}

#[derive(Debug, Clone)]
pub enum Message {
    // Auth
    PasswordInput(String),
    LoginAttempt(bool), // true = root, false = user
    Lock,

    // Navigation
    SwitchWorkspace(Workspace),

    // Chat
    ChatInput(String),
    SendMessage,
}

impl Application for NeuroGraphApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                auth_state: AuthState::new(),
                current_workspace: Workspace::Welcome,
                core: CoreBridge::new(),  // Инициализация Rust core
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("NeuroGraph OS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PasswordInput(password) => {
                self.auth_state.update_password(password);
            }
            Message::LoginAttempt(is_root) => {
                if self.auth_state.try_login(is_root) {
                    self.current_workspace = Workspace::Welcome;
                }
            }
            Message::SwitchWorkspace(workspace) => {
                self.current_workspace = workspace;
            }
            Message::Lock => {
                self.auth_state.lock();
            }
            Message::ChatInput(_) => {
                // Обработается в workspace
            }
            Message::SendMessage => {
                // Обработается в workspace
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        if !self.auth_state.is_authenticated() {
            self.auth_state.view()
        } else {
            self.current_workspace.view(&self.auth_state, &self.core)
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
